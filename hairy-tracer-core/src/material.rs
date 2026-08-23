/// An opaque material identifier.
///
/// This does not contain any shading data — shading is out of scope.
/// It just identifies *which* material was hit so that the future shader
/// can look up the right illumination model / color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterialId(pub u32);
