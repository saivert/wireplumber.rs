// This file was generated by gir (https://github.com/gtk-rs/gir)
// DO NOT EDIT

use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::StaticType;
use glib::Type;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "WpConstraintType")]
pub enum ConstraintType {
    #[doc(alias = "WP_CONSTRAINT_TYPE_NONE")]
    None,
    #[doc(alias = "WP_CONSTRAINT_TYPE_PW_GLOBAL_PROPERTY")]
    PwGlobalProperty,
    #[doc(alias = "WP_CONSTRAINT_TYPE_PW_PROPERTY")]
    PwProperty,
    #[doc(alias = "WP_CONSTRAINT_TYPE_G_PROPERTY")]
    GProperty,
#[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ConstraintType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ConstraintType::{}", match *self {
            Self::None => "None",
            Self::PwGlobalProperty => "PwGlobalProperty",
            Self::PwProperty => "PwProperty",
            Self::GProperty => "GProperty",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl IntoGlib for ConstraintType {
    type GlibType = ffi::WpConstraintType;

    fn into_glib(self) -> ffi::WpConstraintType {
        match self {
            Self::None => ffi::WP_CONSTRAINT_TYPE_NONE,
            Self::PwGlobalProperty => ffi::WP_CONSTRAINT_TYPE_PW_GLOBAL_PROPERTY,
            Self::PwProperty => ffi::WP_CONSTRAINT_TYPE_PW_PROPERTY,
            Self::GProperty => ffi::WP_CONSTRAINT_TYPE_G_PROPERTY,
            Self::__Unknown(value) => value,
}
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpConstraintType> for ConstraintType {
    unsafe fn from_glib(value: ffi::WpConstraintType) -> Self {
        match value {
            ffi::WP_CONSTRAINT_TYPE_NONE => Self::None,
            ffi::WP_CONSTRAINT_TYPE_PW_GLOBAL_PROPERTY => Self::PwGlobalProperty,
            ffi::WP_CONSTRAINT_TYPE_PW_PROPERTY => Self::PwProperty,
            ffi::WP_CONSTRAINT_TYPE_G_PROPERTY => Self::GProperty,
            value => Self::__Unknown(value),
}
    }
}

impl StaticType for ConstraintType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_constraint_type_get_type()) }
    }
}

impl glib::value::ValueType for ConstraintType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for ConstraintType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for ConstraintType {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "WpConstraintVerb")]
pub enum ConstraintVerb {
    #[doc(alias = "WP_CONSTRAINT_VERB_EQUALS")]
    Equals,
    #[doc(alias = "WP_CONSTRAINT_VERB_NOT_EQUALS")]
    NotEquals,
    #[doc(alias = "WP_CONSTRAINT_VERB_IN_LIST")]
    InList,
    #[doc(alias = "WP_CONSTRAINT_VERB_IN_RANGE")]
    InRange,
    #[doc(alias = "WP_CONSTRAINT_VERB_MATCHES")]
    Matches,
    #[doc(alias = "WP_CONSTRAINT_VERB_IS_PRESENT")]
    IsPresent,
    #[doc(alias = "WP_CONSTRAINT_VERB_IS_ABSENT")]
    IsAbsent,
#[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ConstraintVerb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ConstraintVerb::{}", match *self {
            Self::Equals => "Equals",
            Self::NotEquals => "NotEquals",
            Self::InList => "InList",
            Self::InRange => "InRange",
            Self::Matches => "Matches",
            Self::IsPresent => "IsPresent",
            Self::IsAbsent => "IsAbsent",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl IntoGlib for ConstraintVerb {
    type GlibType = ffi::WpConstraintVerb;

    fn into_glib(self) -> ffi::WpConstraintVerb {
        match self {
            Self::Equals => ffi::WP_CONSTRAINT_VERB_EQUALS,
            Self::NotEquals => ffi::WP_CONSTRAINT_VERB_NOT_EQUALS,
            Self::InList => ffi::WP_CONSTRAINT_VERB_IN_LIST,
            Self::InRange => ffi::WP_CONSTRAINT_VERB_IN_RANGE,
            Self::Matches => ffi::WP_CONSTRAINT_VERB_MATCHES,
            Self::IsPresent => ffi::WP_CONSTRAINT_VERB_IS_PRESENT,
            Self::IsAbsent => ffi::WP_CONSTRAINT_VERB_IS_ABSENT,
            Self::__Unknown(value) => value,
}
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpConstraintVerb> for ConstraintVerb {
    unsafe fn from_glib(value: ffi::WpConstraintVerb) -> Self {
        match value {
            ffi::WP_CONSTRAINT_VERB_EQUALS => Self::Equals,
            ffi::WP_CONSTRAINT_VERB_NOT_EQUALS => Self::NotEquals,
            ffi::WP_CONSTRAINT_VERB_IN_LIST => Self::InList,
            ffi::WP_CONSTRAINT_VERB_IN_RANGE => Self::InRange,
            ffi::WP_CONSTRAINT_VERB_MATCHES => Self::Matches,
            ffi::WP_CONSTRAINT_VERB_IS_PRESENT => Self::IsPresent,
            ffi::WP_CONSTRAINT_VERB_IS_ABSENT => Self::IsAbsent,
            value => Self::__Unknown(value),
}
    }
}

impl StaticType for ConstraintVerb {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_constraint_verb_get_type()) }
    }
}

impl glib::value::ValueType for ConstraintVerb {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for ConstraintVerb {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for ConstraintVerb {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "WpDBusState")]
pub enum DBusState {
    #[doc(alias = "WP_DBUS_STATE_CLOSED")]
    Closed,
    #[doc(alias = "WP_DBUS_STATE_CONNECTING")]
    Connecting,
    #[doc(alias = "WP_DBUS_STATE_CONNECTED")]
    Connected,
#[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
impl fmt::Display for DBusState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DBusState::{}", match *self {
            Self::Closed => "Closed",
            Self::Connecting => "Connecting",
            Self::Connected => "Connected",
            _ => "Unknown",
        })
    }
}

#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
#[doc(hidden)]
impl IntoGlib for DBusState {
    type GlibType = ffi::WpDBusState;

    fn into_glib(self) -> ffi::WpDBusState {
        match self {
            Self::Closed => ffi::WP_DBUS_STATE_CLOSED,
            Self::Connecting => ffi::WP_DBUS_STATE_CONNECTING,
            Self::Connected => ffi::WP_DBUS_STATE_CONNECTED,
            Self::__Unknown(value) => value,
}
    }
}

#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
#[doc(hidden)]
impl FromGlib<ffi::WpDBusState> for DBusState {
    unsafe fn from_glib(value: ffi::WpDBusState) -> Self {
        match value {
            ffi::WP_DBUS_STATE_CLOSED => Self::Closed,
            ffi::WP_DBUS_STATE_CONNECTING => Self::Connecting,
            ffi::WP_DBUS_STATE_CONNECTED => Self::Connected,
            value => Self::__Unknown(value),
}
    }
}

#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
impl StaticType for DBusState {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_dbus_state_get_type()) }
    }
}

#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
impl glib::value::ValueType for DBusState {
    type Type = Self;
}

#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
unsafe impl<'a> FromValue<'a> for DBusState {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
impl ToValue for DBusState {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "WpDirection")]
pub enum Direction {
    #[doc(alias = "WP_DIRECTION_INPUT")]
    Input,
    #[doc(alias = "WP_DIRECTION_OUTPUT")]
    Output,
#[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Direction::{}", match *self {
            Self::Input => "Input",
            Self::Output => "Output",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl IntoGlib for Direction {
    type GlibType = ffi::WpDirection;

    fn into_glib(self) -> ffi::WpDirection {
        match self {
            Self::Input => ffi::WP_DIRECTION_INPUT,
            Self::Output => ffi::WP_DIRECTION_OUTPUT,
            Self::__Unknown(value) => value,
}
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpDirection> for Direction {
    unsafe fn from_glib(value: ffi::WpDirection) -> Self {
        match value {
            ffi::WP_DIRECTION_INPUT => Self::Input,
            ffi::WP_DIRECTION_OUTPUT => Self::Output,
            value => Self::__Unknown(value),
}
    }
}

impl StaticType for Direction {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_direction_get_type()) }
    }
}

impl glib::value::ValueType for Direction {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for Direction {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for Direction {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "WpLibraryErrorEnum")]
pub enum LibraryErrorEnum {
    #[doc(alias = "WP_LIBRARY_ERROR_INVARIANT")]
    Invariant,
    #[doc(alias = "WP_LIBRARY_ERROR_INVALID_ARGUMENT")]
    InvalidArgument,
    #[doc(alias = "WP_LIBRARY_ERROR_OPERATION_FAILED")]
    OperationFailed,
#[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for LibraryErrorEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LibraryErrorEnum::{}", match *self {
            Self::Invariant => "Invariant",
            Self::InvalidArgument => "InvalidArgument",
            Self::OperationFailed => "OperationFailed",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl IntoGlib for LibraryErrorEnum {
    type GlibType = ffi::WpLibraryErrorEnum;

    fn into_glib(self) -> ffi::WpLibraryErrorEnum {
        match self {
            Self::Invariant => ffi::WP_LIBRARY_ERROR_INVARIANT,
            Self::InvalidArgument => ffi::WP_LIBRARY_ERROR_INVALID_ARGUMENT,
            Self::OperationFailed => ffi::WP_LIBRARY_ERROR_OPERATION_FAILED,
            Self::__Unknown(value) => value,
}
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpLibraryErrorEnum> for LibraryErrorEnum {
    unsafe fn from_glib(value: ffi::WpLibraryErrorEnum) -> Self {
        match value {
            ffi::WP_LIBRARY_ERROR_INVARIANT => Self::Invariant,
            ffi::WP_LIBRARY_ERROR_INVALID_ARGUMENT => Self::InvalidArgument,
            ffi::WP_LIBRARY_ERROR_OPERATION_FAILED => Self::OperationFailed,
            value => Self::__Unknown(value),
}
    }
}

impl StaticType for LibraryErrorEnum {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_library_error_enum_get_type()) }
    }
}

impl glib::value::ValueType for LibraryErrorEnum {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for LibraryErrorEnum {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for LibraryErrorEnum {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "WpLinkState")]
pub enum LinkState {
    #[doc(alias = "WP_LINK_STATE_ERROR")]
    Error,
    #[doc(alias = "WP_LINK_STATE_UNLINKED")]
    Unlinked,
    #[doc(alias = "WP_LINK_STATE_INIT")]
    Init,
    #[doc(alias = "WP_LINK_STATE_NEGOTIATING")]
    Negotiating,
    #[doc(alias = "WP_LINK_STATE_ALLOCATING")]
    Allocating,
    #[doc(alias = "WP_LINK_STATE_PAUSED")]
    Paused,
    #[doc(alias = "WP_LINK_STATE_ACTIVE")]
    Active,
#[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
impl fmt::Display for LinkState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LinkState::{}", match *self {
            Self::Error => "Error",
            Self::Unlinked => "Unlinked",
            Self::Init => "Init",
            Self::Negotiating => "Negotiating",
            Self::Allocating => "Allocating",
            Self::Paused => "Paused",
            Self::Active => "Active",
            _ => "Unknown",
        })
    }
}

#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
#[doc(hidden)]
impl IntoGlib for LinkState {
    type GlibType = ffi::WpLinkState;

    fn into_glib(self) -> ffi::WpLinkState {
        match self {
            Self::Error => ffi::WP_LINK_STATE_ERROR,
            Self::Unlinked => ffi::WP_LINK_STATE_UNLINKED,
            Self::Init => ffi::WP_LINK_STATE_INIT,
            Self::Negotiating => ffi::WP_LINK_STATE_NEGOTIATING,
            Self::Allocating => ffi::WP_LINK_STATE_ALLOCATING,
            Self::Paused => ffi::WP_LINK_STATE_PAUSED,
            Self::Active => ffi::WP_LINK_STATE_ACTIVE,
            Self::__Unknown(value) => value,
}
    }
}

#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
#[doc(hidden)]
impl FromGlib<ffi::WpLinkState> for LinkState {
    unsafe fn from_glib(value: ffi::WpLinkState) -> Self {
        match value {
            ffi::WP_LINK_STATE_ERROR => Self::Error,
            ffi::WP_LINK_STATE_UNLINKED => Self::Unlinked,
            ffi::WP_LINK_STATE_INIT => Self::Init,
            ffi::WP_LINK_STATE_NEGOTIATING => Self::Negotiating,
            ffi::WP_LINK_STATE_ALLOCATING => Self::Allocating,
            ffi::WP_LINK_STATE_PAUSED => Self::Paused,
            ffi::WP_LINK_STATE_ACTIVE => Self::Active,
            value => Self::__Unknown(value),
}
    }
}

#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
impl StaticType for LinkState {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_link_state_get_type()) }
    }
}

#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
impl glib::value::ValueType for LinkState {
    type Type = Self;
}

#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
unsafe impl<'a> FromValue<'a> for LinkState {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v0_4_11", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_11")))]
impl ToValue for LinkState {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "WpNodeState")]
pub enum NodeState {
    #[doc(alias = "WP_NODE_STATE_ERROR")]
    Error,
    #[doc(alias = "WP_NODE_STATE_CREATING")]
    Creating,
    #[doc(alias = "WP_NODE_STATE_SUSPENDED")]
    Suspended,
    #[doc(alias = "WP_NODE_STATE_IDLE")]
    Idle,
    #[doc(alias = "WP_NODE_STATE_RUNNING")]
    Running,
#[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for NodeState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NodeState::{}", match *self {
            Self::Error => "Error",
            Self::Creating => "Creating",
            Self::Suspended => "Suspended",
            Self::Idle => "Idle",
            Self::Running => "Running",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl IntoGlib for NodeState {
    type GlibType = ffi::WpNodeState;

    fn into_glib(self) -> ffi::WpNodeState {
        match self {
            Self::Error => ffi::WP_NODE_STATE_ERROR,
            Self::Creating => ffi::WP_NODE_STATE_CREATING,
            Self::Suspended => ffi::WP_NODE_STATE_SUSPENDED,
            Self::Idle => ffi::WP_NODE_STATE_IDLE,
            Self::Running => ffi::WP_NODE_STATE_RUNNING,
            Self::__Unknown(value) => value,
}
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpNodeState> for NodeState {
    unsafe fn from_glib(value: ffi::WpNodeState) -> Self {
        match value {
            ffi::WP_NODE_STATE_ERROR => Self::Error,
            ffi::WP_NODE_STATE_CREATING => Self::Creating,
            ffi::WP_NODE_STATE_SUSPENDED => Self::Suspended,
            ffi::WP_NODE_STATE_IDLE => Self::Idle,
            ffi::WP_NODE_STATE_RUNNING => Self::Running,
            value => Self::__Unknown(value),
}
    }
}

impl StaticType for NodeState {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_node_state_get_type()) }
    }
}

impl glib::value::ValueType for NodeState {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for NodeState {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for NodeState {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(any(feature = "v0_4_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_10")))]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "WpSiAdapterPortsState")]
pub enum SiAdapterPortsState {
    #[doc(alias = "WP_SI_ADAPTER_PORTS_STATE_NONE")]
    None,
    #[doc(alias = "WP_SI_ADAPTER_PORTS_STATE_CONFIGURING")]
    Configuring,
    #[doc(alias = "WP_SI_ADAPTER_PORTS_STATE_CONFIGURED")]
    Configured,
#[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v0_4_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_10")))]
impl fmt::Display for SiAdapterPortsState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SiAdapterPortsState::{}", match *self {
            Self::None => "None",
            Self::Configuring => "Configuring",
            Self::Configured => "Configured",
            _ => "Unknown",
        })
    }
}

#[cfg(any(feature = "v0_4_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_10")))]
#[doc(hidden)]
impl IntoGlib for SiAdapterPortsState {
    type GlibType = ffi::WpSiAdapterPortsState;

    fn into_glib(self) -> ffi::WpSiAdapterPortsState {
        match self {
            Self::None => ffi::WP_SI_ADAPTER_PORTS_STATE_NONE,
            Self::Configuring => ffi::WP_SI_ADAPTER_PORTS_STATE_CONFIGURING,
            Self::Configured => ffi::WP_SI_ADAPTER_PORTS_STATE_CONFIGURED,
            Self::__Unknown(value) => value,
}
    }
}

#[cfg(any(feature = "v0_4_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_10")))]
#[doc(hidden)]
impl FromGlib<ffi::WpSiAdapterPortsState> for SiAdapterPortsState {
    unsafe fn from_glib(value: ffi::WpSiAdapterPortsState) -> Self {
        match value {
            ffi::WP_SI_ADAPTER_PORTS_STATE_NONE => Self::None,
            ffi::WP_SI_ADAPTER_PORTS_STATE_CONFIGURING => Self::Configuring,
            ffi::WP_SI_ADAPTER_PORTS_STATE_CONFIGURED => Self::Configured,
            value => Self::__Unknown(value),
}
    }
}

#[cfg(any(feature = "v0_4_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_10")))]
impl StaticType for SiAdapterPortsState {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_si_adapter_ports_state_get_type()) }
    }
}

#[cfg(any(feature = "v0_4_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_10")))]
impl glib::value::ValueType for SiAdapterPortsState {
    type Type = Self;
}

#[cfg(any(feature = "v0_4_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_10")))]
unsafe impl<'a> FromValue<'a> for SiAdapterPortsState {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v0_4_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4_10")))]
impl ToValue for SiAdapterPortsState {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "WpTransitionStep")]
pub enum TransitionStep {
    #[doc(alias = "WP_TRANSITION_STEP_NONE")]
    None,
    #[doc(alias = "WP_TRANSITION_STEP_ERROR")]
    Error,
    #[doc(alias = "WP_TRANSITION_STEP_CUSTOM_START")]
    CustomStart,
#[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for TransitionStep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TransitionStep::{}", match *self {
            Self::None => "None",
            Self::Error => "Error",
            Self::CustomStart => "CustomStart",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl IntoGlib for TransitionStep {
    type GlibType = ffi::WpTransitionStep;

    fn into_glib(self) -> ffi::WpTransitionStep {
        match self {
            Self::None => ffi::WP_TRANSITION_STEP_NONE,
            Self::Error => ffi::WP_TRANSITION_STEP_ERROR,
            Self::CustomStart => ffi::WP_TRANSITION_STEP_CUSTOM_START,
            Self::__Unknown(value) => value,
}
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WpTransitionStep> for TransitionStep {
    unsafe fn from_glib(value: ffi::WpTransitionStep) -> Self {
        match value {
            ffi::WP_TRANSITION_STEP_NONE => Self::None,
            ffi::WP_TRANSITION_STEP_ERROR => Self::Error,
            ffi::WP_TRANSITION_STEP_CUSTOM_START => Self::CustomStart,
            value => Self::__Unknown(value),
}
    }
}

impl StaticType for TransitionStep {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::wp_transition_step_get_type()) }
    }
}

impl glib::value::ValueType for TransitionStep {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for TransitionStep {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for TransitionStep {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

