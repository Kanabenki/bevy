//! Additional [`Gizmos`] Functions -- Grids
//!
//! Includes the implementation of [`Gizmos::grid`] and [`Gizmos::grid_2d`],
//! and assorted support items.

use crate::prelude::{GizmoConfigGroup, Gizmos};
use bevy_math::{Mat2, UVec2};
use bevy_math::{Quat, Vec2, Vec3};
use bevy_render::color::Color;

fn grid_inner(size: Vec2, divisions: UVec2, border: bool) -> impl Iterator<Item = (Vec2, Vec2)> {
    let step_x = size.x / divisions.x as f32;
    let half_x = size.x / 2.0;
    let step_y = size.y / divisions.y as f32;
    let half_y = size.y / 2.0;
    let (start, end) = if border {
        (UVec2::ZERO, divisions + UVec2::ONE)
    } else {
        (UVec2::ONE, divisions)
    };
    (start.x..end.x)
        .map(move |i| {
            (
                Vec2::new(-half_x + i as f32 * step_x, -half_y),
                Vec2::new(-half_x + i as f32 * step_x, half_y),
            )
        })
        .chain((start.y..end.y).map(move |i| {
            (
                Vec2::new(-half_x, -half_y + i as f32 * step_y),
                Vec2::new(half_x, -half_y + i as f32 * step_y),
            )
        }))
}

impl<'w, 's, T: GizmoConfigGroup> Gizmos<'w, 's, T> {
    /// Draw a grid in 3D centered on `position` with a number of cells controlled by `subdivisions`.
    /// Without rotation, the grid will be drawn in the xy plane.
    ///
    /// This should be called for each frame the grid needs to be rendered.
    ///
    /// # Example
    /// ```
    /// # use bevy_gizmos::prelude::*;
    /// # use bevy_render::prelude::*;
    /// # use bevy_math::prelude::*;
    /// fn system(mut gizmos: Gizmos) {
    ///     gizmos.grid(Vec2::new(10.0, 30.0), UVec2::new(5, 10), Vec3::ZERO, Quat::IDENTITY, Color::GREEN);
    ///
    ///     // Disable the outer lines.
    ///     gizmos
    ///         .grid(Vec2::splat(400.0), UVec2::splat(20), Vec3::ZERO, Quat::IDENTITY, Color::GREEN)
    ///         .with_border(false);
    /// }
    /// # bevy_ecs::system::assert_is_system(system);
    /// ```
    #[inline]
    pub fn grid(
        &mut self,
        size: Vec2,
        divisions: UVec2,
        position: Vec3,
        rotation: Quat,
        color: Color,
    ) -> GridBuilder<'_, 'w, 's, T> {
        GridBuilder {
            gizmos: self,
            position,
            divisions,
            border: true,
            rotation,
            size,
            color,
        }
    }

    /// Draw a grid in 2D centered on `position` with a number of cells controlled by `subdivisions`.
    ///
    /// This should be called for each frame the grid needs to be rendered.
    ///
    /// # Example
    /// ```
    /// # use bevy_gizmos::prelude::*;
    /// # use bevy_render::prelude::*;
    /// # use bevy_math::prelude::*;
    /// fn system(mut gizmos: Gizmos) {
    ///     gizmos.grid_2d(Vec2::new(10.0, 30.0), UVec2::new(5, 10), Vec2::ZERO, 0.0, Color::GREEN);
    ///
    ///     // Disable the outer lines.
    ///     gizmos
    ///         .grid_2d(Vec2::splat(400.0), UVec2::splat(20), Vec2::ZERO, 0.0, Color::GREEN)
    ///         .with_border(false);
    /// }
    /// # bevy_ecs::system::assert_is_system(system);
    /// ```
    #[inline]
    pub fn grid_2d(
        &mut self,
        size: Vec2,
        divisions: UVec2,
        position: Vec2,
        angle: f32,
        color: Color,
    ) -> Grid2dBuilder<'_, 'w, 's, T> {
        Grid2dBuilder {
            gizmos: self,
            size,
            divisions,
            border: true,
            position,
            rotation: Mat2::from_angle(angle),
            color,
        }
    }
}

/// A builder returned by [`Gizmos::grid`].
pub struct GridBuilder<'a, 'w, 's, T: GizmoConfigGroup> {
    gizmos: &'a mut Gizmos<'w, 's, T>,
    position: Vec3,
    divisions: UVec2,
    border: bool,
    rotation: Quat,
    size: Vec2,
    color: Color,
}

impl<T: GizmoConfigGroup> GridBuilder<'_, '_, '_, T> {
    /// Whether to draw the outer lines of the grid.
    pub fn with_border(&mut self, border: bool) {
        self.border = border;
    }
}

impl<T: GizmoConfigGroup> Drop for GridBuilder<'_, '_, '_, T> {
    fn drop(&mut self) {
        if !self.gizmos.enabled {
            return;
        }

        let positions = grid_inner(self.size, self.divisions, self.border).map(|(start, end)| {
            (
                self.rotation * start.extend(0.0) + self.position,
                self.rotation * end.extend(0.0) + self.position,
            )
        });
        for (start, end) in positions {
            self.gizmos.line(start, end, self.color);
        }
    }
}

/// A builder returned by [`Gizmos::grid_2d`].
pub struct Grid2dBuilder<'a, 'w, 's, T: GizmoConfigGroup> {
    gizmos: &'a mut Gizmos<'w, 's, T>,
    position: Vec2,
    divisions: UVec2,
    border: bool,
    rotation: Mat2,
    size: Vec2,
    color: Color,
}

impl<T: GizmoConfigGroup> Grid2dBuilder<'_, '_, '_, T> {
    /// Whether to draw the outer lines of the grid.
    pub fn with_border(&mut self, border: bool) {
        self.border = border;
    }
}

impl<T: GizmoConfigGroup> Drop for Grid2dBuilder<'_, '_, '_, T> {
    fn drop(&mut self) {
        if !self.gizmos.enabled {
            return;
        };

        let positions = grid_inner(self.size, self.divisions, self.border).map(|(start, end)| {
            (
                self.rotation * start + self.position,
                self.rotation * end + self.position,
            )
        });
        for (start, end) in positions {
            self.gizmos.line_2d(start, end, self.color);
        }
    }
}
