#[derive(Debug, Clone, Copy)]
/// Enum of the possible strategies to write messages in the CSV.
pub enum SendStrategy {
    /// Writes messages from the same neighborhood to the same CSV file.
    /// One file per neighborhood.
    NeighborhoodWise,
}
