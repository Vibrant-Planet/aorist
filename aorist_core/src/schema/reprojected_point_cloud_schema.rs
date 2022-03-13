use crate::asset::*;
use crate::attributes::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::template::*;
use aorist_attributes::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{attribute, derived_schema};
use derivative::Derivative;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

derived_schema! {
    name: ReprojectedPointCloudSchema,
    sources:
      - point_cloud: PointCloudAsset,
    attributes:
      X: Int64("X", false),
      Y: Int64("Y", false),
      Z: Int64("Z", false),
      intensity: NaturalNumber("Intensity", true),
      bit_fields: Int64("Bitflags: ReturnNumber, NumberOfReturns, ScanDirection, EdgeOfFlightLine", false),
      raw_classification: Factor("Classification", false),
      scan_angle_rank: IntegerNumber("Scan angle rank (-90 to + 90) - Left side", false),
      user_data: IntegerNumber("User data", true),
      point_source_id: NumericIdentifier("Point source ID", false),
      gps_time: FloatNumber("GPS time", false),
      red: IntegerNumber("Red channel", false),
      green: IntegerNumber("Green channel", false),
      blue: IntegerNumber("Blue channel", false)
}
