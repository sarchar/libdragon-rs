use crate::*;

/// Wrapper around [`model64_anim_slot_t`](libdragon_sys::model64_anim_slot_t).
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Model64AnimSlot {
    Slot0,
    Slot1,
    Slot2,
    Slot3
}

impl Into<libdragon_sys::model64_anim_slot_t> for Model64AnimSlot {
    fn into(self) -> libdragon_sys::model64_anim_slot_t {
        match self {
            Model64AnimSlot::Slot0 => libdragon_sys::model64_anim_slot_t_MODEL64_ANIM_SLOT_0,
            Model64AnimSlot::Slot1 => libdragon_sys::model64_anim_slot_t_MODEL64_ANIM_SLOT_1,
            Model64AnimSlot::Slot2 => libdragon_sys::model64_anim_slot_t_MODEL64_ANIM_SLOT_2,
            Model64AnimSlot::Slot3 => libdragon_sys::model64_anim_slot_t_MODEL64_ANIM_SLOT_3,
        }
    }
}

/// Wrapper around [`model64_t`](libdragon_sys::model64_t)
pub struct Model64<'a> {
    ptr: *mut libdragon_sys::model64_t,
    phantom: core::marker::PhantomData<&'a u8>,
}

/// Wrapper around [`mesh_t`](libdragon_sys::mesh_t)
pub struct Mesh<'a> {
    ptr: *mut libdragon_sys::mesh_t,
    phantom: core::marker::PhantomData<&'a u8>,
}

/// Wrapper around [`model64_model_t`](libdragon_sys::model64_model_t)
pub struct Model64Node<'a> {
    ptr: *mut libdragon_sys::model64_node_t,
    model_ptr: *mut libdragon_sys::model64_t,
    phantom: core::marker::PhantomData<&'a u8>,
}

/// Wrapper around [`primitive_t`](libdragon_sys::primitive_t)
pub struct Primitive<'a> {
    ptr: *mut libdragon_sys::primitive_t,
    phantom: core::marker::PhantomData<&'a u8>,
}

impl<'a> Model64<'a> {
    /// Load a Model64 from the filesystem
    ///
    /// See [`model64_load`](libdragon_sys::model64_load) for details.
    pub fn load(path: dfs::DfsPathBuf) -> Result<Self> {
        let path_bytes: &[u8] = path.as_bytes();
        let cpath = CString::new(path_bytes).unwrap();

        let ptr = unsafe {
            libdragon_sys::model64_load(cpath.as_ptr())
        };

        if ptr == ::core::ptr::null_mut() {
            // TODO a different error code?
            return Err(LibDragonError::DfsError{ error: dfs::DfsError::NotFound });
        }

        Ok(Model64 {
            ptr: ptr,
            phantom: core::marker::PhantomData,
        })
    }

    /// Load a sprite from a buffer
    ///
    /// See [`model64_load_buf`](libdragon_sys::model64_load_buf) for details.
    pub fn load_buf<'b, T>(buf: &'b mut [T]) -> Model64<'b> {
        let ptr = unsafe {
            libdragon_sys::model64_load_buf(buf.as_mut_ptr() as *mut _, (buf.len() * core::mem::size_of::<T>()) as i32)
        };
        
        Model64 {
            ptr: ptr,
            phantom: core::marker::PhantomData,
        }
    }

    /// Clone a Model64
    ///
    /// See [`model64_clone`](libdragon_sys::model64_clone) for details.
    pub fn clone<'b>(&self) -> Model64<'b> {
        let ptr = unsafe {
            libdragon_sys::model64_clone(self.ptr)
        };

        Model64 {
            ptr: ptr,
            phantom: core::marker::PhantomData,
        }
    }

    /// Return the number of meshes in this model.
    ///
    /// See [`model64_get_mesh_count`](libdragon_sys::model64_get_mesh_count) for details.
    #[inline]
    pub fn get_mesh_count(&self) -> usize { unsafe { libdragon_sys::model64_get_mesh_count(self.ptr) as usize } }

    /// Return the mesh at the specified index
    ///
    /// See [`model64_get_mesh`](libdragon_sys::model64_get_mesh) for details.
    #[inline]
    pub fn get_mesh(&self, mesh_index: usize) -> Mesh<'a> { 
        let ptr = unsafe { 
            libdragon_sys::model64_get_mesh(self.ptr, mesh_index as u32) 
        };

        Mesh {
            ptr: ptr,
            phantom: core::marker::PhantomData,
        }
    }

    /// Return the number of nodees in this model.
    ///
    /// See [`model64_get_node_count`](libdragon_sys::model64_get_node_count) for details.
    #[inline]
    pub fn get_node_count(&self) -> usize { unsafe { libdragon_sys::model64_get_node_count(self.ptr) as usize } }

    /// Return the node at the specified index
    ///
    /// See [`model64_get_node`](libdragon_sys::model64_get_node) for details.
    #[inline]
    pub fn get_node(&self, node_index: usize) -> Model64Node<'a> { 
        let ptr = unsafe { 
            libdragon_sys::model64_get_node(self.ptr, node_index as u32) 
        };

        Model64Node {
            ptr: ptr,
            model_ptr: self.ptr,
            phantom: core::marker::PhantomData,
        }
    }

    /// Return the first node with the specified name in the model.
    #[inline]
    pub fn search_node(&self, name: &str) -> Option<Model64Node<'a>> {
        let c_name = CString::new(name).unwrap();
        let ptr = unsafe {
            libdragon_sys::model64_search_node(self.ptr, c_name.as_ptr())
        };

        if ptr == core::ptr::null_mut() {
            return None;
        }
        Some(Model64Node {
            ptr: ptr,
            model_ptr: self.ptr,
            phantom: core::marker::PhantomData,
        })
    }

    /// Draw an entire model.
    ///
    /// See [`model64_draw`](libdragon_sys::model64_draw) for details.
    #[inline] pub fn draw(&self) { unsafe { libdragon_sys::model64_draw(self.ptr); } }

    /// Play an animation.
    /// 
    /// See [`model64_anim_play`](libdragon_sys::model64_anim_play) for details.
    #[inline] pub fn anim_play(&mut self, anim: &str, slot: Model64AnimSlot, paused: bool, start_time: f32) {
        let c_anim = CString::new(anim).unwrap();
        unsafe {
            libdragon_sys::model64_anim_play(self.ptr, c_anim.as_ptr(), slot.into(), paused, start_time);
        }
    }

    /// Stop an animation.
    /// 
    /// See [`model64_anim_stop`](libdragon_sys::model64_anim_stop) for details.
    #[inline] pub fn anim_stop(&mut self, slot: Model64AnimSlot) {
        unsafe {
            libdragon_sys::model64_anim_stop(self.ptr, slot.into());
        }
    }

    /// Get the length of an animation in seconds.
    /// 
    /// See [`model64_anim_get_length`](libdragon_sys::model64_anim_get_length) for details.
    #[inline] pub fn get_length(&self, anim: &str) -> f32 {
        let c_anim = CString::new(anim).unwrap();
        unsafe {
            libdragon_sys::model64_anim_get_length(self.ptr, c_anim.as_ptr())
        }
    }

    /// Get the current time of an animation in seconds.
    /// 
    /// See [`model64_anim_get_time`](libdragon_sys::model64_anim_get_time) for details.
    #[inline] pub fn get_time(&self, slot: Model64AnimSlot) -> f32 {
        unsafe {
            libdragon_sys::model64_anim_get_time(self.ptr, slot.into())
        }
    }

    /// Set the current time of an animation in seconds.
    /// 
    /// See [`model64_anim_set_time`](libdragon_sys::model64_anim_set_time) for details.
    #[inline] pub fn set_time(&mut self, slot: Model64AnimSlot, time: f32) {
        unsafe {
            libdragon_sys::model64_anim_set_time(self.ptr, slot.into(), time);
        }
    }

    /// Set the playback speed of an animation
    /// 
    /// See [`model64_anim_set_speed`](libdragon_sys::model64_anim_set_speed) for details.
    #[inline] pub fn set_speed(&mut self, slot: Model64AnimSlot, speed: f32) {
        unsafe {
            libdragon_sys::model64_anim_set_speed(self.ptr, slot.into(), speed);
        }
    }

    /// Set the looping flag of an animation
    /// 
    /// See [`model64_anim_set_loop`](libdragon_sys::model64_anim_set_loop) for details.
    #[inline] pub fn set_loop(&mut self, slot: Model64AnimSlot, loop_: bool) {
        unsafe {
            libdragon_sys::model64_anim_set_loop(self.ptr, slot.into(), loop_);
        }
    }

    /// Set the paused flag of an animation
    /// 
    /// See [`model64_anim_set_pause`](libdragon_sys::model64_anim_set_pause) for details.
    #[inline] pub fn set_pause(&mut self, slot: Model64AnimSlot, pause: bool) {
        unsafe {
            libdragon_sys::model64_anim_set_pause(self.ptr, slot.into(), pause);
        }
    }

    /// Update a model
    ///
    /// See [`model64_update`](libdragon_sys::model64_update) for details.
    #[inline] pub fn update(&mut self, deltatime: f32) {
        unsafe { 
            libdragon_sys::model64_update(self.ptr, deltatime);
        }
    }
}

impl Drop for Model64<'_> {
    /// Free the Model64 memory
    ///
    /// See [`model64_free`](libdragon_sys::model64_free) for details.
    fn drop(&mut self) { unsafe { libdragon_sys::model64_free(self.ptr); } }
}

impl<'a> Mesh<'a> {
    /// Return the number of primitives in this mesh.
    ///
    /// See [`model64_get_primitive_count`](libdragon_sys::model64_get_primitive_count) for details.
    #[inline] pub fn get_primitive_count(&self) -> u32 { unsafe { libdragon_sys::model64_get_primitive_count(self.ptr) } }

    /// Return the primitive at the specified index.
    ///
    /// See [`model64_get_primitive`](libdragon_sys::model64_get_primitive) for details.
    #[inline]
    pub fn get_primtive(&self, primitive_index: u32) -> Primitive<'a> {
        let ptr = unsafe {
            libdragon_sys::model64_get_primitive(self.ptr, primitive_index)
        };
        Primitive {
            ptr: ptr,
            phantom: core::marker::PhantomData,
        }
    }

    /// Draw a single mesh
    /// 
    /// See [`model64_draw_mesh`](libdragon_sys::model64_draw_mesh) for details.
    #[inline] pub fn draw(&self) { unsafe { libdragon_sys::model64_draw_mesh(self.ptr); } }
}

impl<'a> Model64Node<'a> {
    /// Sets the position of a node in a model relative to its parent
    ///
    /// See [`model64_set_node_pos`](libdragon_sys::model64_set_node_pos) for details.
    #[inline]
    pub fn set_node_pos(&mut self, x: f32, y: f32, z: f32) {
        unsafe {
            libdragon_sys::model64_set_node_pos(self.model_ptr, self.ptr, x, y, z);
        }
    }

    /// Sets the rotation of a node in a model relative to its parent in the form of an euler angle
    /// (ZYX rotation order) in radians.
    ///
    /// See [`model64_set_node_rot`](libdragon_sys::model64_set_node_rot) for details.
    #[inline]
    pub fn set_node_rot(&mut self, x: f32, y: f32, z: f32) {
        unsafe {
            libdragon_sys::model64_set_node_rot(self.model_ptr, self.ptr, x, y, z);
        }
    }

    /// Sets the rotation of a node in a model relative to its parent in the form of a quaternion.
    ///
    /// See [`model64_set_node_rot_quat`](libdragon_sys::model64_set_node_rot_quat) for details.
    #[inline]
    pub fn set_node_rot_quat(&mut self, x: f32, y: f32, z: f32, w: f32) {
        unsafe {
            libdragon_sys::model64_set_node_rot_quat(self.model_ptr, self.ptr, x, y, z, w);
        }
    }

    /// Sets the scale of a node in a model relative to its parent.
    ///
    /// See [`model64_set_node_scale`](libdragon_sys::model64_set_node_scale) for details.
    #[inline]
    pub fn set_node_scale(&mut self, x: f32, y: f32, z: f32) {
        unsafe {
            libdragon_sys::model64_set_node_scale(self.model_ptr, self.ptr, x, y, z);
        }
    }

    /// Gets the transformation matrix between a model's root node and a node in a model.
    ///
    /// See [`model64_get_node_world_mtx`](libdragon_sys::model64_get_node_world_mtx) for details.
    #[inline]
    pub fn get_node_world_mtx(&self) -> Vec<f32> {
        let mut mtx = Vec::with_capacity(16);
        unsafe {
            libdragon_sys::model64_get_node_world_mtx(self.model_ptr, self.ptr, mtx.as_mut_ptr());
        }
        mtx
    }

    /// Draw a single node
    ///
    /// See [`model64_draw_node`](libdragon_sys::model64_draw_node) for details.
    #[inline] pub fn draw(&self) { unsafe { libdragon_sys::model64_draw_node(self.model_ptr, self.ptr); } }
}

impl<'a> Primitive<'a> {
    /// Draw a single primitive
    ///
    /// See [`model64_draw_primitive`](libdragon_sys::model64_draw_primitive) for details.
    #[inline] pub fn draw(&self) { unsafe { libdragon_sys::model64_draw_primitive(self.ptr); } }
}
