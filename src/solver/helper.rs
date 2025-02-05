use crate::geometry::ParticlesContacts;
use crate::kernel::Kernel;
use crate::math::Real;
use crate::object::{Boundary, Fluid};

#[cfg(feature = "parallel")]
use rayon::prelude::*;
use crate::counters::Counters;

pub fn update_fluid_contacts<KernelDensity: Kernel, KernelGradient: Kernel>(
    kernel_radius: Real,
    fluid_fluid_contacts: &mut [ParticlesContacts],
    fluid_boundary_contacts: &mut [ParticlesContacts],
    fluids: &[Fluid],
    boundaries: &[Boundary],
) {
    for contacts in fluid_fluid_contacts.iter_mut() {
        par_iter_mut!(contacts.contacts_mut()).for_each(|contacts| {
            for c in contacts.get_mut().unwrap() {
                let fluid1 = &fluids[c.i_model];
                let fluid2 = &fluids[c.j_model];
                let pi = fluid1.positions[c.i];
                let pj = fluid2.positions[c.j];

                c.weight = KernelDensity::points_apply(&pi, &pj, kernel_radius);
                c.gradient = KernelGradient::points_apply_diff1(&pi, &pj, kernel_radius);
            }
        })
    }

    for contacts in fluid_boundary_contacts.iter_mut() {
        par_iter_mut!(contacts.contacts_mut()).for_each(|contacts| {
            for c in contacts.get_mut().unwrap() {
                let fluid1 = &fluids[c.i_model];
                let bound2 = &boundaries[c.j_model];

                let pi = fluid1.positions[c.i];
                let pj = bound2.positions[c.j];

                c.weight = KernelDensity::points_apply(&pi, &pj, kernel_radius);
                c.gradient = KernelGradient::points_apply_diff1(&pi, &pj, kernel_radius);
            }
        })
    }
}

pub fn update_boundary_contacts<KernelDensity: Kernel, KernelGradient: Kernel>(
    counters: &mut Counters,
    kernel_radius: Real,
    boundary_boundary_contacts: &mut [ParticlesContacts],
    boundaries: &[Boundary],
) {
    let mut weights_log = String::new();
    for contacts in boundary_boundary_contacts.iter_mut() {
        if should_skip_weight_computation(contacts) {
            continue;
        }
        par_iter_mut!(contacts.contacts_mut()).for_each(|contacts| {
            for c in contacts.get_mut().unwrap() {
                let bound1 = &boundaries[c.i_model];
                let bound2 = &boundaries[c.j_model];

                let pi = bound1.positions[c.i];
                let pj = bound2.positions[c.j];

                c.weight = KernelDensity::points_apply(&pi, &pj, kernel_radius);
                c.gradient = KernelGradient::points_apply_diff1(&pi, &pj, kernel_radius);

                // weights_log.push_str(&format!("{} ", c.weight));
            }
        })
    }

    // counters.log(weights_log.as_str());
}

#[cfg(feature = "opt-weight")]
fn should_skip_weight_computation(contacts: &mut ParticlesContacts) -> bool {
    contacts.contacts().first().unwrap().read().unwrap().first().unwrap().weight != na::zero::<Real>()
}

#[cfg(not(feature = "opt-weight"))]
fn should_skip_weight_computation(contacts: &mut ParticlesContacts) -> bool {
    // Default behavior
    false
}
