# Unreleased

### Added

### Changed

-   get rid of too expressive error variants
-   improve parsing performance
-   changed tagging algorithm
    (tagged IDs now encode to completely different strings)

### Fixed

# 0.1.3 (28 July, 2024)

### Added

-   reexport of aes
-   zeroize feature

### Changed

-   pad encoded IDs to fixed length

### Fixed

# 0.1.2 (July 22, 2024)

### Added

-   crate documentation
-   std feature for `std::error::Error` implementation

### Changed

-   encoding of IDs changed to base58
-   derive `PartialEq, Eq` for `Id`

### Fixed

-   integer overflow on decoding of a string
