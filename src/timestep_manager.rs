#![allow(dead_code)] // NOTE: keep this until we fix CFL

use approx::AbsDiffEq;
use num::Zero;

use crate::math::Real;
use crate::object::Fluid;

/// Structure responsible for regulating the timestep length of the simulation.
pub struct TimestepManager {
    cfl_coeff: Real,
    min_num_substeps: u32,
    max_num_substeps: u32,
    dt: Real,
    inv_dt: Real,
    total_step_size: Real,
    remaining_time: Real,
    particle_radius: Real,
}

impl TimestepManager {
    /// Initialize a new timestep manager with default parameters.
    pub fn new(particle_radius: Real) -> Self {
        Self {
            cfl_coeff: na::convert::<_, Real>(0.4),
            min_num_substeps: 1,
            max_num_substeps: 10,
            particle_radius,
            dt: na::zero::<Real>(),
            inv_dt: na::zero::<Real>(),
            total_step_size: na::zero::<Real>(),
            remaining_time: na::zero::<Real>(),
        }
    }

    fn max_substep(&self, fluids: &[Fluid]) -> Real {
        let mut max_sq_vel = na::zero::<Real>();
        for (v, a) in fluids
            .iter()
            .flat_map(|f| f.velocities.iter().zip(f.accelerations.iter()))
        {
            max_sq_vel = max_sq_vel.max((v + a * self.remaining_time).norm_squared());
        }

        self.particle_radius * na::convert::<_, Real>(2.0) / max_sq_vel.sqrt() * self.cfl_coeff
    }

    /// Resets the remaining time of the timestep manager.
    pub fn reset(&mut self, total_step_size: Real) {
        self.total_step_size = total_step_size;
        self.remaining_time = total_step_size;
    }

    /// Checks if all the time of this timestep has been consumed.
    #[inline]
    pub fn is_done(&self) -> bool {
        self.remaining_time <= Real::default_epsilon()
    }

    /// The current substep length.
    #[inline]
    pub fn dt(&self) -> Real {
        self.dt
    }

    /// The inverse of the current substep length.
    ///
    /// If the substep length is zero, this inverse is also zero.
    #[inline]
    pub fn inv_dt(&self) -> Real {
        self.inv_dt
    }

    /// Advance to the next substep.
    #[inline]
    pub fn advance(&mut self, fluids: &[Fluid]) {
        let substep = self.compute_substep(fluids);
        self.dt = substep;
        self.inv_dt = if substep.is_zero() {
            na::zero::<Real>()
        } else {
            na::one::<Real>() / substep
        };
        self.remaining_time -= self.dt;
    }

    fn compute_substep(&self, _fluids: &[Fluid]) -> Real {
        return self.total_step_size;
        // FIXME
        //        let min_substep = self.total_step_size / na::convert::<_, Real>(self.max_num_substeps as f64);
        //        let max_substep = self.total_step_size / na::convert::<_, Real>(self.min_num_substeps as f64);
        //        let computed_substep = self.max_substep(fluids);
        //        na::clamp(computed_substep, min_substep, max_substep)
    }
}
