use std::fmt::Display;

/// Represents a CTGP version number with major, minor, revision, and optional subrevision components.
///
/// Versions are formatted as `major.minor.revision` or `major.minor.revision-subrevision`
/// when a subrevision is present (e.g. `1.03.1066-9`).
#[derive(Clone, Copy, Debug)]
pub struct CTGPVersion {
    /// Major version number.
    major: u8,
    /// Minor version number.
    minor: u8,
    /// Revision number.
    revision: u16,
    /// Optional subrevision number, present in most release versions.
    subrevision: Option<u8>,
}

impl CTGPVersion {
    /// Creates a new [`CTGPVersion`] with the given components.
    ///
    /// # Arguments
    ///
    /// * `major` - Major version number.
    /// * `minor` - Minor version number.
    /// * `revision` - Revision number.
    /// * `subrevision` - Optional subrevision number.
    pub fn new(major: u8, minor: u8, revision: u16, subrevision: Option<u8>) -> Self {
        Self {
            major,
            minor,
            revision,
            subrevision,
        }
    }

    /// Attempts to map the 4-byte CTGP CORE version to one or more possible release versions.
    ///
    /// Because multiple CTGP releases can share the same internal byte tag, this returns a
    /// [`Vec`] of all [`CTGPVersion`]s that are consistent with the given bytes. Returns
    /// [`None`] if the byte pattern is not recognised.
    ///
    /// # Arguments
    ///
    /// * `bytes` - A 4-byte slice containing the raw CTGP CORE version.
    ///
    /// # Returns
    ///
    /// * `Some(Vec<CTGPVersion>)` - One or more possible release versions matching the CORE version.
    /// * `None` - The CORE version does not correspond to any known CTGP release.
    ///
    /// # Examples
    ///
    /// ```
    /// use rkg_utils::footer::ctgp_footer::CTGPVersion;
    ///
    /// // Unambiguous mapping
    /// let versions = CTGPVersion::from(&[0x01, 0x03, 0x01, 0x42]).unwrap();
    /// assert_eq!(versions.len(), 1);
    ///
    /// // Ambiguous mapping — multiple releases share this tag
    /// let versions = CTGPVersion::from(&[0x01, 0x03, 0x01, 0x50]).unwrap();
    /// assert_eq!(versions.len(), 3);
    ///
    /// // Unknown tag
    /// assert!(CTGPVersion::from(&[0xFF, 0xFF, 0xFF, 0xFF]).is_none());
    /// ```
    pub fn from(bytes: &[u8]) -> Option<Vec<Self>> {
        let mut possible_versions = Vec::new();
        match bytes {
            &[0x01, 0x03, 0x01, 0x42] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1044, None));
            }

            &[0x01, 0x03, 0x01, 0x44] | &[0x01, 0x03, 0x01, 0x46] | &[0x01, 0x03, 0x01, 0x48] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1046, Some(5)));
            }

            &[0x01, 0x03, 0x01, 0x50] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1048, Some(5)));
                possible_versions.push(CTGPVersion::new(1, 3, 1050, Some(4)));
                possible_versions.push(CTGPVersion::new(1, 3, 1054, Some(5)));
            }

            &[0x01, 0x03, 0x01, 0x78] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1058, Some(6)));
            }

            &[0x01, 0x03, 0x01, 0x80] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1060, Some(6)));
            }

            &[0x01, 0x03, 0x01, 0x82] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1062, Some(4)));
                possible_versions.push(CTGPVersion::new(1, 3, 1062, Some(5)));
            }

            &[0x01, 0x03, 0x01, 0x92] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1064, Some(6)));
            }

            &[0x01, 0x03, 0x02, 0x00] => {
                for subrevision in 5..=8 {
                    possible_versions.push(CTGPVersion::new(1, 3, 1066, Some(subrevision)));
                }
            }

            &[0x01, 0x03, 0x02, 0x02] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1066, Some(9)));
            }

            &[0x01, 0x03, 0x02, 0x16] => {
                for subrevision in 10..=15 {
                    possible_versions.push(CTGPVersion::new(1, 3, 1066, Some(subrevision)));
                }
            }

            &[0x01, 0x03, 0x02, 0x18] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1068, Some(5)));
                possible_versions.push(CTGPVersion::new(1, 3, 1070, Some(2)));
            }

            &[0x01, 0x03, 0x02, 0x26] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1070, Some(2)));
            }

            &[0x01, 0x03, 0x02, 0x28] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1074, Some(6)));
            }

            &[0x01, 0x03, 0x02, 0x40] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1074, Some(9)));
            }

            &[0x01, 0x03, 0x02, 0x42] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1074, Some(10)));
            }

            &[0x01, 0x03, 0x02, 0x44] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1074, Some(11)));
            }

            &[0x01, 0x03, 0x02, 0x52] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1076, Some(6)));
            }

            &[0x01, 0x03, 0x02, 0x56] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1076, Some(9)));
            }

            &[0x01, 0x03, 0x02, 0x78] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1076, Some(12)));
                possible_versions.push(CTGPVersion::new(1, 3, 1076, Some(15)));
            }

            &[0x01, 0x03, 0x02, 0x84] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1080, Some(8)));
                possible_versions.push(CTGPVersion::new(1, 3, 1080, Some(9)));
                possible_versions.push(CTGPVersion::new(1, 3, 1082, Some(3)));
            }

            &[0x01, 0x03, 0x02, 0x86] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1084, Some(3)));
                possible_versions.push(CTGPVersion::new(1, 3, 1084, Some(5)));

                possible_versions.push(CTGPVersion::new(1, 3, 1086, Some(4)));
                possible_versions.push(CTGPVersion::new(1, 3, 1088, Some(3)));
                possible_versions.push(CTGPVersion::new(1, 3, 1090, Some(4)));
                possible_versions.push(CTGPVersion::new(1, 3, 1092, Some(2)));
            }

            &[0x01, 0x03, 0x02, 0x92] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1094, Some(4)));
            }

            &[0x01, 0x03, 0x02, 0x96] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1094, Some(8)));
            }

            &[0x01, 0x03, 0x03, 0x00] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1094, Some(12)));
                possible_versions.push(CTGPVersion::new(1, 3, 1096, Some(4)));
                possible_versions.push(CTGPVersion::new(1, 3, 1098, Some(3)));
            }

            &[0x01, 0x03, 0x03, 0x12] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1100, Some(5)));
                possible_versions.push(CTGPVersion::new(1, 3, 1100, Some(8)));
                possible_versions.push(CTGPVersion::new(1, 3, 1100, Some(10)));
                possible_versions.push(CTGPVersion::new(1, 3, 1102, Some(4)));
                possible_versions.push(CTGPVersion::new(1, 3, 1104, Some(4)));
                possible_versions.push(CTGPVersion::new(1, 3, 1106, Some(3)));
            }

            &[0x01, 0x03, 0x03, 0x14] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1106, Some(6)));
                possible_versions.push(CTGPVersion::new(1, 3, 1108, Some(4)));
                possible_versions.push(CTGPVersion::new(1, 3, 1112, Some(4)));
            }

            &[0x01, 0x03, 0x03, 0x38] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1112, Some(8)));
                possible_versions.push(CTGPVersion::new(1, 3, 1112, Some(10)));
                possible_versions.push(CTGPVersion::new(1, 3, 1112, Some(12)));
            }

            &[0x01, 0x03, 0x03, 0x42] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1114, Some(4)));
                possible_versions.push(CTGPVersion::new(1, 3, 1116, Some(2)));
                possible_versions.push(CTGPVersion::new(1, 3, 1118, Some(4)));
                possible_versions.push(CTGPVersion::new(1, 3, 1120, Some(5)));
            }

            &[0x01, 0x03, 0x03, 0x44] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1120, Some(7)));
            }

            &[0x01, 0x03, 0x03, 0x46] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1120, Some(10)));
                possible_versions.push(CTGPVersion::new(1, 3, 1122, Some(4)));
                possible_versions.push(CTGPVersion::new(1, 3, 1124, Some(4)));
                possible_versions.push(CTGPVersion::new(1, 3, 1126, Some(4)));
                possible_versions.push(CTGPVersion::new(1, 3, 1128, Some(3)));
            }

            &[0x01, 0x03, 0x03, 0x72] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1130, Some(6)));
            }

            &[0x01, 0x03, 0x03, 0x74] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1130, Some(8)));
                possible_versions.push(CTGPVersion::new(1, 3, 1132, Some(4)));
                possible_versions.push(CTGPVersion::new(1, 3, 1134, Some(2)));
                possible_versions.push(CTGPVersion::new(1, 3, 1136, Some(4)));
            }

            &[0x01, 0x03, 0x03, 0x78] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1138, Some(4)));
                possible_versions.push(CTGPVersion::new(1, 3, 1138, Some(7)));
                possible_versions.push(CTGPVersion::new(1, 3, 1140, Some(4)));
                possible_versions.push(CTGPVersion::new(1, 3, 1142, Some(4)));
                possible_versions.push(CTGPVersion::new(1, 3, 1144, Some(4)));
            }

            &[0x01, 0x03, 0x03, 0x90] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1150, Some(9)));
            }

            &[0x01, 0x03, 0x03, 0x92] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1150, Some(11)));
                possible_versions.push(CTGPVersion::new(1, 3, 1154, Some(5)));
            }

            &[0x01, 0x03, 0x03, 0x94] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1156, Some(6)));
            }

            &[0x01, 0x03, 0x03, 0x96] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1156, Some(9)));
                possible_versions.push(CTGPVersion::new(1, 3, 1158, Some(3)));
            }

            &[0x01, 0x03, 0x03, 0x98] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1158, Some(5)));
            }

            &[0x01, 0x03, 0x04, 0x00] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1158, Some(7)));
                possible_versions.push(CTGPVersion::new(1, 3, 1160, Some(5)));
                possible_versions.push(CTGPVersion::new(1, 3, 1162, Some(3)));
                possible_versions.push(CTGPVersion::new(1, 3, 1166, Some(5)));
                possible_versions.push(CTGPVersion::new(1, 3, 1170, Some(5)));
            }

            &[0x01, 0x03, 0x04, 0x02] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1172, Some(5)));
            }

            &[0x01, 0x03, 0x04, 0x04] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1172, Some(7)));
            }

            &[0x01, 0x03, 0x04, 0x06] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1176, Some(6)));
                possible_versions.push(CTGPVersion::new(1, 3, 1178, Some(3)));
            }

            &[0x01, 0x03, 0x04, 0x08] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1178, Some(5)));
            }

            &[0x01, 0x03, 0x04, 0x10] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1178, Some(10)));
                possible_versions.push(CTGPVersion::new(1, 3, 1180, Some(5)));
                possible_versions.push(CTGPVersion::new(1, 3, 1180, Some(7)));
                possible_versions.push(CTGPVersion::new(1, 3, 1182, Some(4)));
                possible_versions.push(CTGPVersion::new(1, 3, 1186, Some(5)));
                possible_versions.push(CTGPVersion::new(1, 3, 1188, Some(3)));
                possible_versions.push(CTGPVersion::new(1, 3, 1190, Some(5)));
                possible_versions.push(CTGPVersion::new(1, 3, 1192, Some(3)));
            }

            &[0x01, 0x03, 0x04, 0x12] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1192, Some(5)));
            }

            _ => {
                return None;
            }
        }
        Some(possible_versions)
    }

    /// Constructs a [`CTGPVersion`] representing the CORE version.
    ///
    /// Each byte is interpreted as a two-digit BCD (binary-coded decimal) value.
    /// The first two bytes form the major and minor version numbers, and the last
    /// two bytes are combined to form the revision number. No subrevision is set.
    ///
    /// # Arguments
    ///
    /// * `bytes` - A slice of at least 4 bytes containing the raw CORE version.
    ///
    /// # Errors
    ///
    /// Returns a [`std::num::ParseIntError`] if any of the formatted byte strings
    /// cannot be parsed as an integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use rkg_utils::footer::ctgp_footer::CTGPVersion;
    ///
    /// let version = CTGPVersion::core_from(&[0x01, 0x03, 0x04, 0x10]).unwrap();
    /// assert_eq!(version.to_string(), "1.03.0410");
    /// ```
    pub fn core_from(bytes: &[u8]) -> Result<Self, std::num::ParseIntError> {
        let major: u8 = format!("{:02X}", bytes[0]).parse()?;
        let minor: u8 = format!("{:02X}", bytes[1]).parse()?;
        let revision: u16 = format!("{:02X}{:02X}", bytes[2], bytes[3]).parse()?;

        Ok(Self::new(major, minor, revision, None))
    }

    /// Returns the 4-byte BCD-encoded CORE version representation, the inverse of [`CTGPVersion::core_from`].
    pub fn to_core_bytes(&self) -> [u8; 4] {
        let rev_str = format!("{:04}", self.revision);
        [
            u8::from_str_radix(&format!("{:02}", self.major), 16).unwrap_or(0),
            u8::from_str_radix(&format!("{:02}", self.minor), 16).unwrap_or(0),
            u8::from_str_radix(&rev_str[0..2], 16).unwrap_or(0),
            u8::from_str_radix(&rev_str[2..4], 16).unwrap_or(0),
        ]
    }
}

/// Formats the version as `major.minor.revision` or `major.minor.revision-subrevision`
/// when a subrevision is present.
///
/// Minor is zero-padded to 2 digits and revision to 4 digits, e.g. `1.03.1066-9`.
impl Display for CTGPVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(subrevision) = self.subrevision {
            write!(
                f,
                "{}.{:02}.{:04}-{}",
                self.major, self.minor, self.revision, subrevision
            )
        } else {
            write!(f, "{}.{:02}.{:04}", self.major, self.minor, self.revision)
        }
    }
}
