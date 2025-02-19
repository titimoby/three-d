use crate::core::texture::*;
use crate::core::*;

///
/// A array of 2D color textures that can be rendered into.
///
/// **Note:** [DepthTest] is disabled if not also writing to a depth texture array.
/// Use a [RenderTargetArray] to write to both color and depth.
///
#[deprecated = "Use Texture2DArray instead"]
pub type ColorTargetTexture2DArray<T> = Texture2DArray<T>;

///
/// A array of 2D color textures that can be rendered into.
///
/// **Note:** [DepthTest] is disabled if not also writing to a depth texture array.
/// Use a [RenderTargetArray] to write to both color and depth.
///
pub struct Texture2DArray<T: TextureDataType> {
    context: Context,
    id: crate::context::Texture,
    width: u32,
    height: u32,
    depth: u32,
    number_of_mip_maps: u32,
    format: Format,
    _dummy: T,
}

impl<T: TextureDataType> Texture2DArray<T> {
    ///
    /// Creates a new array of 2D textures.
    ///
    pub fn new_empty(
        context: &Context,
        width: u32,
        height: u32,
        depth: u32,
        min_filter: Interpolation,
        mag_filter: Interpolation,
        mip_map_filter: Option<Interpolation>,
        wrap_s: Wrapping,
        wrap_t: Wrapping,
        format: Format,
    ) -> ThreeDResult<Self> {
        let id = generate(context)?;
        let number_of_mip_maps = calculate_number_of_mip_maps(mip_map_filter, width, height, None);
        set_parameters(
            context,
            &id,
            consts::TEXTURE_2D_ARRAY,
            min_filter,
            mag_filter,
            if number_of_mip_maps == 1 {
                None
            } else {
                mip_map_filter
            },
            wrap_s,
            wrap_t,
            None,
        );
        context.bind_texture(consts::TEXTURE_2D_ARRAY, &id);
        context.tex_storage_3d(
            consts::TEXTURE_2D_ARRAY,
            number_of_mip_maps,
            T::internal_format(format)?,
            width,
            height,
            depth,
        );
        Ok(Self {
            context: context.clone(),
            id,
            width,
            height,
            depth,
            number_of_mip_maps,
            format,
            _dummy: T::default(),
        })
    }

    ///
    /// Renders whatever rendered in the `render` closure into the textures defined by the input parameters `color_layers`.
    /// Output at location *i* defined in the fragment shader is written to the color texture layer at the *ith* index in `color_layers`.
    /// Before writing, the textures are cleared based on the given clear state.
    ///
    /// **Note:** [DepthTest] is disabled if not also writing to a depth texture array.
    /// Use a [RenderTargetArray] to write to both color and depth.
    ///
    pub fn write<F: FnOnce() -> ThreeDResult<()>>(
        &mut self,
        color_layers: &[u32],
        clear_state: ClearState,
        render: F,
    ) -> ThreeDResult<()> {
        RenderTargetArray::new_color(&self.context.clone(), self)?.write(
            color_layers,
            0,
            clear_state,
            render,
        )
    }

    /// The width of this texture.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The height of this texture.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// The number of layers.
    pub fn depth(&self) -> u32 {
        self.depth
    }

    /// The format of this texture.
    pub fn format(&self) -> Format {
        self.format
    }

    pub(in crate::core) fn generate_mip_maps(&self) {
        if self.number_of_mip_maps > 1 {
            self.context
                .bind_texture(consts::TEXTURE_2D_ARRAY, &self.id);
            self.context.generate_mipmap(consts::TEXTURE_2D_ARRAY);
        }
    }

    pub(in crate::core) fn bind_as_color_target(&self, layer: u32, channel: u32) {
        self.context.framebuffer_texture_layer(
            consts::DRAW_FRAMEBUFFER,
            consts::COLOR_ATTACHMENT0 + channel,
            &self.id,
            0,
            layer,
        );
    }
}

impl<T: TextureDataType> Texture for Texture2DArray<T> {
    fn bind(&self, location: u32) {
        bind_at(&self.context, &self.id, consts::TEXTURE_2D_ARRAY, location);
    }
}

impl<T: TextureDataType> Drop for Texture2DArray<T> {
    fn drop(&mut self) {
        self.context.delete_texture(&self.id);
    }
}
