use crate::code_block::CodeBlock;
use crate::concept::{Concept, ConceptAncestry};
use crate::constraint::{AoristConstraint, Constraint};
use crate::constraint_block::ConstraintBlock;
use crate::constraint_state::{AncestorRecord, ConstraintState};
use crate::data_setup::Universe;
use crate::endpoints::EndpointConfig;
use crate::etl_singleton::ETLDAG;
use crate::object::TAoristObject;
use crate::python::ParameterTuple;
use aorist_primitives::{Bash, Dialect, Presto, Python};
use inflector::cases::snakecase::to_snake_case;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{HashMap, HashSet, VecDeque};
use std::marker::PhantomData;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use uuid::Uuid;

pub struct Driver<'a, D>
where
    D: ETLDAG,
{
    pub concepts: Arc<RwLock<HashMap<(Uuid, String), Concept<'a>>>>,
    constraints: HashMap<(Uuid, String), Arc<RwLock<Constraint>>>,
    satisfied_constraints: HashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a>>>>,
    blocks: Vec<ConstraintBlock<'a, D::T>>,
    // map from: constraint_name => (dependent_constraint_names, constraints_by_uuid)
    unsatisfied_constraints: HashMap<
        String,
        (
            HashSet<String>,
            HashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a>>>>,
        ),
    >,
    ancestry: Arc<ConceptAncestry<'a>>,
    dag_type: PhantomData<D>,
    endpoints: EndpointConfig,
    constraint_explanations: HashMap<String, (Option<String>, Option<String>)>,
}

impl<'a, D> Driver<'a, D>
where
    D: ETLDAG,
{
    fn compute_all_ancestors(
        universe: Concept<'a>,
        concept_map: &HashMap<(Uuid, String), Concept<'a>>,
    ) -> HashMap<(Uuid, String), Vec<AncestorRecord>> {
        let mut ancestors: HashMap<(Uuid, String), Vec<AncestorRecord>> = HashMap::new();
        let mut frontier: Vec<AncestorRecord> = Vec::new();
        frontier.push(AncestorRecord::new(
            universe.get_uuid(),
            universe.get_type(),
            universe.get_tag(),
            universe.get_index_as_child(),
        ));
        ancestors.insert(
            (universe.get_uuid(), universe.get_type()),
            vec![AncestorRecord::new(
                universe.get_uuid(),
                universe.get_type(),
                None,
                0,
            )],
        );
        while frontier.len() > 0 {
            let mut new_frontier: Vec<AncestorRecord> = Vec::new();
            for child in frontier.drain(0..) {
                let key = child.get_key();
                let concept = concept_map.get(&key).unwrap();
                let child_ancestors = ancestors.get(&key).unwrap().clone();
                for grandchild in concept.get_child_concepts() {
                    let t = AncestorRecord::new(
                        grandchild.get_uuid(),
                        grandchild.get_type(),
                        grandchild.get_tag(),
                        grandchild.get_index_as_child(),
                    );
                    new_frontier.push(t.clone());
                    let mut grandchild_ancestors = child_ancestors.clone();
                    grandchild_ancestors.push(t);
                    ancestors.insert(
                        (grandchild.get_uuid(), grandchild.get_type()),
                        grandchild_ancestors,
                    );
                }
            }
            frontier = new_frontier;
        }
        ancestors
    }
    fn get_unsatisfied_constraints(
        constraints: &HashMap<(Uuid, String), Arc<RwLock<Constraint>>>,
        concepts: Arc<RwLock<HashMap<(Uuid, String), Concept<'a>>>>,
        ancestors: &HashMap<(Uuid, String), Vec<AncestorRecord>>,
    ) -> HashMap<
        String,
        (
            HashSet<String>,
            HashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a>>>>,
        ),
    > {
        let mut raw_unsatisfied_constraints: HashMap<
            (Uuid, String),
            Arc<RwLock<ConstraintState<'a>>>,
        > = constraints
            .iter()
            .map(|(k, rw)| {
                (
                    k.clone(),
                    Arc::new(RwLock::new(ConstraintState::new(
                        rw.clone(),
                        concepts.clone(),
                        ancestors,
                    ))),
                )
            })
            .collect();

        /* Remove redundant dependencies */
        // constraint key => constraint key dependency on it
        let mut changes_made = true;
        while changes_made {
            changes_made = false;
            let mut reverse_dependencies: LinkedHashMap<(Uuid, String), Vec<(Uuid, String)>> =
                LinkedHashMap::new();
            for (k, v) in raw_unsatisfied_constraints.iter() {
                for dep in v.read().unwrap().unsatisfied_dependencies.iter() {
                    reverse_dependencies
                        .entry(dep.clone())
                        .or_insert(Vec::new())
                        .push(k.clone());
                }
            }

            let tips = raw_unsatisfied_constraints
                .iter()
                .filter(|(k, _v)| !reverse_dependencies.contains_key(k));
            for tip in tips {
                let mut visits: HashMap<(Uuid, String), (Uuid, String)> = HashMap::new();
                let mut queue: VecDeque<((Uuid, String), Arc<RwLock<_>>)> = VecDeque::new();
                queue.push_back((tip.0.clone(), tip.1.clone()));
                while queue.len() > 0 {
                    let (key, elem) = queue.pop_front().unwrap();
                    let new_deps = elem.read().unwrap().unsatisfied_dependencies.clone();
                    for dep in new_deps {
                        let dep_constraint = raw_unsatisfied_constraints.get(&dep).unwrap().clone();
                        // we have already visited this dependency
                        if visits.contains_key(&dep) {
                            // this is the key of the constraint through which we have already visited
                            let prev = visits.remove(&dep).unwrap();
                            if prev != key {
                                let prev_constraint =
                                    raw_unsatisfied_constraints.get(&prev).unwrap();
                                let mut write = prev_constraint.write().unwrap();
                                if write.get_name() != elem.read().unwrap().get_name() {
                                    assert!(write.unsatisfied_dependencies.remove(&dep));
                                    changes_made = true;
                                }
                            }
                        }
                        visits.insert(dep.clone(), key.clone());
                        queue.push_back((dep, dep_constraint));
                    }
                }
            }
        }
        /* Remove dangling dummy tasks */
        let mut changes_made = true;
        while changes_made {
            changes_made = false;
            let dangling = raw_unsatisfied_constraints
                .iter()
                .filter(|(_k, v)| {
                    v.read().unwrap().unsatisfied_dependencies.len() == 0
                        && !v.read().unwrap().requires_program()
                })
                .map(|(k, _)| k.clone())
                .collect::<Vec<_>>();
            let mut reverse_dependencies: LinkedHashMap<(Uuid, String), Vec<(Uuid, String)>> =
                LinkedHashMap::new();
            for (k, v) in raw_unsatisfied_constraints.iter() {
                for dep in v.read().unwrap().unsatisfied_dependencies.iter() {
                    reverse_dependencies
                        .entry(dep.clone())
                        .or_insert(Vec::new())
                        .push(k.clone());
                }
            }
            for k in dangling {
                assert!(raw_unsatisfied_constraints.remove(&k).is_some());
                for rev in reverse_dependencies.get(&k).unwrap() {
                    assert!(raw_unsatisfied_constraints
                        .get(rev)
                        .unwrap()
                        .write()
                        .unwrap()
                        .unsatisfied_dependencies
                        .remove(&k));
                }
                changes_made = true;
            }
        }
        /* Remove superfluous dummy tasks */
        loop {
            let superfluous = raw_unsatisfied_constraints
                .iter()
                .filter(|(_k, v)| {
                    v.read().unwrap().unsatisfied_dependencies.len() == 1
                        && !v.read().unwrap().requires_program()
                })
                .map(|(k, _)| k.clone())
                .collect::<Vec<_>>();
            if let Some(elem) = superfluous.into_iter().next() {
                let mut reverse_dependencies: LinkedHashMap<(Uuid, String), Vec<(Uuid, String)>> =
                    LinkedHashMap::new();
                for (k, v) in raw_unsatisfied_constraints.iter() {
                    for dep in v.read().unwrap().unsatisfied_dependencies.iter() {
                        reverse_dependencies
                            .entry(dep.clone())
                            .or_insert(Vec::new())
                            .push(k.clone());
                    }
                }
                let arc = raw_unsatisfied_constraints.remove(&elem).unwrap();
                let dep = arc
                    .read()
                    .unwrap()
                    .unsatisfied_dependencies
                    .iter()
                    .next()
                    .unwrap()
                    .clone();

                if let Some(rev_deps) = reverse_dependencies.get(&elem) {
                    for rev in rev_deps.iter() {
                        let mut write = raw_unsatisfied_constraints
                            .get(rev)
                            .unwrap()
                            .write()
                            .unwrap();
                        assert!(write.unsatisfied_dependencies.remove(&elem));
                        write.unsatisfied_dependencies.insert(dep.clone());
                    }
                }
            } else {
                break;
            }
        }

        let mut unsatisfied_constraints: HashMap<
            String,
            (
                HashSet<String>,
                HashMap<(Uuid, String), Arc<RwLock<ConstraintState>>>,
            ),
        > = AoristConstraint::get_required_constraint_names()
            .into_iter()
            .map(|(k, v)| (k, (v.into_iter().collect(), HashMap::new())))
            .collect();

        for ((uuid, root_type), rw) in raw_unsatisfied_constraints.into_iter() {
            let constraint_name = rw.read().unwrap().get_name();
            unsatisfied_constraints
                .get_mut(&constraint_name)
                .unwrap()
                .1
                .insert((uuid, root_type), rw);
        }

        unsatisfied_constraints
    }
    pub fn new(
        universe: &'a Universe,
        topline_constraint_names: LinkedHashSet<String>,
        debug: bool,
    ) -> Driver<'a, D> {
        let mut concept_map: HashMap<(Uuid, String), Concept<'a>> = HashMap::new();
        let concept = Concept::Universe((universe, 0, None));
        concept.populate_child_concept_map(&mut concept_map);

        let ancestors = Self::compute_all_ancestors(concept, &concept_map);
        let mut family_trees: HashMap<(Uuid, String), HashMap<String, HashSet<Uuid>>> =
            HashMap::new();
        for (key, ancestor_v) in ancestors.iter() {
            for record in ancestor_v {
                family_trees
                    .entry(key.clone())
                    .or_insert(HashMap::new())
                    .entry(record.object_type.clone())
                    .or_insert(HashSet::new())
                    .insert(record.uuid);
            }
            for record in ancestor_v {
                let (uuid, object_type) = key;
                let ancestor_key = record.get_key();
                family_trees
                    .entry(ancestor_key)
                    .or_insert(HashMap::new())
                    .entry(object_type.clone())
                    .or_insert(HashSet::new())
                    .insert(uuid.clone());
            }
            let (uuid, object_type) = key;
            family_trees
                .entry(key.clone())
                .or_insert(HashMap::new())
                .entry(object_type.clone())
                .or_insert(HashSet::new())
                .insert(uuid.clone());
        }

        let mut by_object_type: HashMap<String, Vec<Concept<'a>>> = HashMap::new();
        for ((_uuid, object_type), concept) in concept_map.clone() {
            by_object_type
                .entry(object_type)
                .or_insert(Vec::new())
                .push(concept.clone());
        }
        let mut visited_constraint_names: LinkedHashSet<String> = LinkedHashSet::new();
        // constraint_name => root_id => constraint_object
        let mut generated_constraints: LinkedHashMap<
            String,
            LinkedHashMap<(Uuid, String), Arc<RwLock<Constraint>>>,
        > = LinkedHashMap::new();

        let mut builders = AoristConstraint::builders()
            .into_iter()
            .map(|x| (x.get_constraint_name(), x))
            .collect::<LinkedHashMap<String, _>>();

        let mut builder_q = topline_constraint_names
            .into_iter()
            .map(|x| (x.clone(), builders.remove(&x).unwrap()))
            .collect::<VecDeque<_>>();

        let mut relevant_builders = LinkedHashMap::new();
        let mut visited = HashSet::new();
        let mut g: LinkedHashMap<String, LinkedHashSet<String>> = LinkedHashMap::new();
        let mut rev: HashMap<String, Vec<String>> = HashMap::new();

        while builder_q.len() > 0 {
            let (key, builder) = builder_q.pop_front().unwrap();
            let edges = g.entry(key.clone()).or_insert(LinkedHashSet::new());
            for req in builder.get_required_constraint_names() {
                if !visited.contains(&req) {
                    let another = builders.remove(&req).unwrap();
                    builder_q.push_back((req.clone(), another));
                    visited.insert(req.clone());
                }
                edges.insert(req.clone());
                let rev_edges = rev.entry(req.clone()).or_insert(Vec::new());
                rev_edges.push(key.clone());
            }
            relevant_builders.insert(key.clone(), builder);
        }

        let mut sorted_builders = Vec::new();
        while g.len() > 0 {
            let leaf = g
                .iter()
                .filter(|(_, v)| v.len() == 0)
                .map(|(k, _)| k)
                .next()
                .unwrap()
                .clone();

            let builder = relevant_builders.remove(&leaf).unwrap();
            if let Some(parents) = rev.remove(&leaf) {
                for parent in parents {
                    g.get_mut(&parent).unwrap().remove(&leaf);
                }
            }
            sorted_builders.push(builder);
            g.remove(&leaf);
        }

        let concepts = Arc::new(RwLock::new(concept_map));
        let ancestry: ConceptAncestry<'a> = ConceptAncestry {
            parents: concepts.clone(),
        };

        for builder in sorted_builders {
            let root_object_type = builder.get_root_type_name();
            let constraint_name = builder.get_constraint_name();

            if let Some(root_concepts) = by_object_type.get(&root_object_type) {
                if debug {
                    println!(
                        "Attaching constraint {} to {} objects of type {}.",
                        constraint_name,
                        root_concepts.len(),
                        root_object_type
                    );
                }

                for root in root_concepts {
                    let root_key = (root.get_uuid(), root.get_type());
                    let family_tree = family_trees.get(&root_key).unwrap();
                    let potential_child_constraints = builder
                        .get_required_constraint_names()
                        .into_iter()
                        .map(|req| generated_constraints.get(&req))
                        .filter(|x| x.is_some())
                        .map(|x| {
                            x.unwrap()
                                .iter()
                                .filter(
                                    |((potential_root_id, potential_root_type), _constraint)| {
                                        match family_tree.get(potential_root_type) {
                                            None => false,
                                            Some(set) => set.contains(potential_root_id),
                                        }
                                    },
                                )
                                .map(|(_, constraint)| constraint.clone())
                        })
                        .flatten()
                        .collect::<Vec<Arc<RwLock<Constraint>>>>();
                    if builder.should_add(root.clone()) {
                        let constraint =
                            builder.build_constraint(root.get_uuid(), potential_child_constraints);
                        let gen_for_constraint = generated_constraints
                            .entry(constraint_name.clone())
                            .or_insert(LinkedHashMap::new());
                        assert!(!gen_for_constraint.contains_key(&root_key));
                        gen_for_constraint.insert(root_key, Arc::new(RwLock::new(constraint)));
                    }
                }
            }
            for req in builder.get_required_constraint_names() {
                assert!(visited_constraint_names.contains(&req));
            }
            visited_constraint_names.insert(constraint_name.clone());
        }

        let constraints = generated_constraints
            .into_iter()
            .map(|(_, v)| v.into_iter())
            .flatten()
            .map(|((_root_id, root_type), rw)| {
                (
                    (rw.read().unwrap().get_uuid().clone(), root_type),
                    rw.clone(),
                )
            })
            .collect();

        let unsatisfied_constraints =
            Self::get_unsatisfied_constraints(&constraints, concepts.clone(), &ancestors);

        Self {
            concepts,
            constraints: constraints.clone(),
            satisfied_constraints: HashMap::new(),
            unsatisfied_constraints,
            ancestry: Arc::new(ancestry),
            blocks: Vec::new(),
            dag_type: PhantomData,
            endpoints: universe.endpoints.clone(),
            constraint_explanations: AoristConstraint::get_explanations(),
        }
    }

    fn find_satisfiable_constraint_block(
        &mut self,
    ) -> Option<(
        HashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a>>>>,
        String,
    )> {
        let constraint_block_name = self
            .unsatisfied_constraints
            .iter()
            .filter(|(_, v)| v.0.len() == 0)
            .map(|(k, _)| k.clone())
            .next();
        match constraint_block_name {
            Some(name) => {
                let (_dependency_names, constraints) =
                    self.unsatisfied_constraints.remove(&name).unwrap();
                for (_, (v, _)) in self.unsatisfied_constraints.iter_mut() {
                    v.remove(&name);
                }
                Some((constraints, name))
            }
            None => None,
        }
    }

    fn process_constraint_with_program(
        &mut self,
        constraint: RwLockReadGuard<'_, Constraint>,
        uuid: (Uuid, String),
        calls: &mut HashMap<(String, String, String), Vec<(String, ParameterTuple)>>,
        state: Arc<RwLock<ConstraintState<'a>>>,
    ) {
        let name = constraint.get_name().clone();
        drop(constraint);
        let preferences = vec![
            Dialect::Python(Python {}),
            Dialect::Presto(Presto {}),
            Dialect::Bash(Bash {}),
        ];

        let mut write = state.write().unwrap();
        // TODO: remove dummy hash map
        write.satisfy(&preferences, self.ancestry.clone());
        drop(write);

        // TODO: preambles and calls are superflous
        let key = state.read().unwrap().key.as_ref().unwrap().clone();
        calls
            .entry((
                state.read().unwrap().get_call().unwrap(),
                name,
                uuid.1.clone(),
            ))
            .or_insert(Vec::new())
            .push((key, state.read().unwrap().get_params().unwrap()));
    }
    fn process_constraint_state(
        &mut self,
        uuid: (Uuid, String),
        state: Arc<RwLock<ConstraintState<'a>>>,
        calls: &mut HashMap<(String, String, String), Vec<(String, ParameterTuple)>>,
        reverse_dependencies: &HashMap<(Uuid, String), HashSet<(String, Uuid, String)>>,
    ) {
        let mut write = state.write().unwrap();
        let ancestors = write.get_ancestors();
        write.compute_task_name(&ancestors);
        // TODO: rename this function, it's confusing (this represents
        // constraitn name, key is root name)
        assert!(!write.satisfied);
        assert_eq!(write.unsatisfied_dependencies.len(), 0);
        drop(write);

        let rw = self.constraints.get(&uuid).unwrap().clone();
        let constraint = rw.read().unwrap();
        if constraint.requires_program() {
            self.process_constraint_with_program(constraint, uuid.clone(), calls, state.clone());
        }

        if let Some(v) = reverse_dependencies.get(&uuid) {
            for (dependency_name, dependency_uuid, dependency_root_type) in v {
                let rw = self
                    .unsatisfied_constraints
                    .get(dependency_name)
                    .unwrap()
                    .1
                    .get(&(*dependency_uuid, dependency_root_type.clone()))
                    .unwrap();
                let mut write = rw.write().unwrap();
                write.satisfied_dependencies.push(state.clone());
                assert!(write.unsatisfied_dependencies.remove(&uuid));
                drop(write);
            }
        }

        let mut write = state.write().unwrap();
        write.satisfied = true;
        drop(write);
    }

    fn process_constraint_block(
        &mut self,
        block: &mut HashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a>>>>,
        reverse_dependencies: &HashMap<(Uuid, String), HashSet<(String, Uuid, String)>>,
        constraint_name: String,
    ) -> Vec<CodeBlock<'a, D::T>> {
        // (call, constraint_name, root_name) => (uuid, call parameters)
        let mut calls: HashMap<(String, String, String), Vec<(String, ParameterTuple)>> =
            HashMap::new();
        let mut blocks: Vec<CodeBlock<'a, D::T>> = Vec::new();
        let mut by_dialect: HashMap<Option<Dialect>, Vec<Arc<RwLock<ConstraintState<'a>>>>> =
            HashMap::new();
        for (id, state) in block.clone() {
            self.process_constraint_state(
                id.clone(),
                state.clone(),
                &mut calls,
                reverse_dependencies,
            );
            self.satisfied_constraints.insert(id, state.clone());
            by_dialect
                .entry(state.read().unwrap().get_dialect())
                .or_insert(Vec::new())
                .push(state.clone());
        }
        for (dialect, satisfied) in by_dialect.into_iter() {
            let block = CodeBlock::new(dialect, satisfied, constraint_name.clone());
            blocks.push(block);
        }

        blocks
    }
    fn get_shorter_task_name(task_name: String) -> String {
        let splits = task_name
            .split("__")
            .map(|x| x.to_string())
            .filter(|x| x.len() > 0)
            .collect::<Vec<String>>();
        let mut new_name = task_name.to_string();
        if splits.len() > 2 {
            new_name = format!(
                "{}__{}",
                splits[0].to_string(),
                splits[2..]
                    .iter()
                    .map(|x| x.clone())
                    .collect::<Vec<String>>()
                    .join("__")
            )
            .to_string();
        } else if splits.len() == 2 {
            new_name = splits[0].to_string();
        } else {
            let splits_inner = splits[0]
                .split("_")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            if splits_inner.len() > 2 {
                new_name = format!(
                    "{}_{}",
                    splits_inner[0].to_string(),
                    splits_inner[2..]
                        .iter()
                        .map(|x| x.clone())
                        .collect::<Vec<String>>()
                        .join("_")
                )
                .to_string();
            }
        }
        new_name
    }
    pub fn shorten_task_names(&mut self) {
        let mut task_names: Vec<(String, Arc<RwLock<ConstraintState<'a>>>)> = Vec::new();
        // shorten task names
        for constraint in self.satisfied_constraints.values() {
            let fqn = constraint.read().unwrap().get_fully_qualified_task_name();
            task_names.push((fqn, constraint.clone()));
        }
        loop {
            let mut changes_made = false;

            let mut proposed_names: Vec<String> = task_names.iter().map(|x| x.0.clone()).collect();
            let mut new_task_names: HashSet<String> = proposed_names.clone().into_iter().collect();
            for i in 0..task_names.len() {
                let task_name = proposed_names.get(i).unwrap().clone();
                let new_name = Self::get_shorter_task_name(task_name.clone());
                if new_name != task_name
                    && !new_task_names.contains(&new_name)
                    && proposed_names
                        .iter()
                        .enumerate()
                        .filter(|(pos, x)| *pos != i && x.contains(&new_name))
                        .collect::<Vec<_>>()
                        .len()
                        == 0
                {
                    changes_made = true;
                    new_task_names.insert(new_name.clone());
                    proposed_names[i] = new_name;
                }
            }
            if !changes_made {
                break;
            }
            for i in 0..task_names.len() {
                task_names[i].0 = proposed_names[i].clone();
            }
        }
        for (name, rw) in task_names {
            let mut write = rw.write().unwrap();
            write.set_task_name(name.replace("____", "__"));
        }
    }
    pub fn run(&'a mut self) -> pyo3::PyResult<String> {
        let mut reverse_dependencies: HashMap<(Uuid, String), HashSet<(String, Uuid, String)>> =
            HashMap::new();
        for (name, (_, constraints)) in &self.unsatisfied_constraints {
            for ((uuid, root_type), state) in constraints {
                for (dependency_uuid, dependency_root_type) in
                    &state.read().unwrap().unsatisfied_dependencies
                {
                    reverse_dependencies
                        .entry((*dependency_uuid, dependency_root_type.clone()))
                        .or_insert(HashSet::new())
                        .insert((name.clone(), *uuid, root_type.clone()));
                }
            }
        }

        // find at least one satisfiable constraint
        loop {
            let mut satisfiable = self.find_satisfiable_constraint_block();
            if let Some((ref mut block, ref constraint_name)) = satisfiable {
                let snake_case_name = to_snake_case(constraint_name);
                let members = self.process_constraint_block(
                    &mut block.clone(),
                    &reverse_dependencies,
                    snake_case_name.clone(),
                );
                let (title, body) = self
                    .constraint_explanations
                    .get(constraint_name)
                    .unwrap()
                    .clone();
                // TODO: snake case name can be moved to ConstraintBlock
                let constraint_block = ConstraintBlock::new(snake_case_name, title, body, members);
                self.blocks.push(constraint_block);
            } else {
                break;
            }
        }
        self.shorten_task_names();

        let etl = D::new();
        assert_eq!(self.unsatisfied_constraints.len(), 0);
        let statements_and_preambles = self
            .blocks
            .iter()
            .map(|x| x.get_statements(&self.endpoints))
            .collect::<Vec<_>>();
        return etl.materialize(statements_and_preambles);
    }
}
