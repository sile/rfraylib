#[derive(Debug)]
pub struct RenderTexture {
    pub(crate) inner: raylib4_sys::RenderTexture, // TODO
}

#[derive(Debug)]
pub struct Texture {
    pub(crate) inner: raylib4_sys::Texture, // TODO
}

impl Texture {
    pub(crate) fn to_raw(&self) -> raylib4_sys::Texture {
        self.inner.clone()
    }
}
