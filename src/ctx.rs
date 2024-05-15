#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: u64,
}

// ═════════════════════════════ CONSTRUCTOR ═════════════════════════════
impl Ctx {
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }
}

// ═════════════════════════ PROPERTY ACCESSORS ══════════════════════
impl Ctx {
    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}