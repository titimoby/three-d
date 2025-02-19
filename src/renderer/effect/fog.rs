use crate::core::*;

///
/// An effect that simulates fog, ie. the area where it is applied gets hazy when objects are far away.
///
pub struct FogEffect {
    /// The color of the fog.
    pub color: Color,
    /// The density of the fog.
    pub density: f32,
    /// Determines the variation on the density as a function of time.
    pub animation: f32,
    image_effect: ImageEffect,
}

impl FogEffect {
    ///
    /// Constructs a new fog effect.
    ///
    pub fn new(
        context: &Context,
        color: Color,
        density: f32,
        animation: f32,
    ) -> ThreeDResult<FogEffect> {
        Ok(FogEffect {
            color,
            density,
            animation,
            image_effect: ImageEffect::new(context, include_str!("shaders/fog.frag"))?,
        })
    }

    ///
    /// Apply the fog effect on the current render target based on the given depth map.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write].
    ///
    pub fn apply(
        &self,
        camera: &Camera,
        depth_texture: &DepthTargetTexture2D,
        time: f32,
    ) -> ThreeDResult<()> {
        let render_states = RenderStates {
            write_mask: WriteMask::COLOR,
            blend: Blend::TRANSPARENCY,
            cull: Cull::Back,
            ..Default::default()
        };

        self.image_effect.use_texture("depthMap", depth_texture)?;
        self.image_effect.use_uniform(
            "viewProjectionInverse",
            (camera.projection() * camera.view()).invert().unwrap(),
        )?;
        self.image_effect
            .use_uniform("fogColor", self.color.to_vec3())?;
        self.image_effect.use_uniform("fogDensity", self.density)?;
        self.image_effect.use_uniform("animation", self.animation)?;
        self.image_effect.use_uniform("time", 0.001 * time)?;
        self.image_effect
            .use_uniform("eyePosition", camera.position())?;

        self.image_effect.apply(render_states, camera.viewport())?;
        Ok(())
    }
}
