use bevy::{
    prelude::*,
    render::{RenderApp, extract_component::ExtractComponent},
};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum VelloExtractStep {
    // Extract renderable types, e.g. SVG, Lottie, Text, Scenes
    ExtractAssets,
    // Measure frame
    RunDiagnostics,
}

/// A screenspace render target. We use a resizable fullscreen quad.
#[derive(Component, Default, Clone, ExtractComponent)]
#[extract_app(RenderApp)]
pub struct VelloRenderTarget(pub Handle<Image>);
