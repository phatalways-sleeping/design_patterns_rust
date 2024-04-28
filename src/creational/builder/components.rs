pub trait Component {}

pub trait Roof: Component {}

pub trait Door: Component {}

pub trait Window: Component {}

pub trait Floor: Component {}

pub struct NormalRoof;

impl Roof for NormalRoof {}

impl Component for NormalRoof {}

pub struct NormalDoor;

impl Door for NormalDoor {}

impl Component for NormalDoor {}

pub struct NormalWindow;

impl Window for NormalWindow {}

impl Component for NormalWindow {}

pub struct NormalFloor;

impl Floor for NormalFloor {}

impl Component for NormalFloor {}
