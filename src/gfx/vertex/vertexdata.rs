struct VertexData {
    vbo: VboHandle,

}

impl VertexData {
    /// New vertex data with a brand new VBO. Not reccomended, try to share VBOs.
    pub fn with_new_buffer<T: VertexArray>(vertices: T) -> VertexData {
        let vbo = vertices.to_vbo();
    }
}
