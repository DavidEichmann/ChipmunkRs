
use libc::{c_double, c_int, c_uint};

// Bindings to the CHipmunk 7.0.1 library

#[repr(C)]
#[derive(Clone,Copy)]
pub struct CPVect {
    pub x: c_double,
    pub y: c_double,
}

impl CPVect {
    pub fn new(x: f64, y: f64) -> CPVect { CPVect { x: x, y: y } }
}

pub type CPFloat = c_double;
pub enum CPSpace {}
pub enum CPBody {}
pub enum CPShape {}
pub enum CPCircleShape {}
pub enum CPConstraint {}

#[link(name="chipmunk")]
extern "C" {
    // Space
    pub fn cpSpaceNew() -> *const CPSpace;
    pub fn cpSpaceFree(CPSpace: *const CPSpace);

    // Space Settings
    pub fn cpSpaceIsLocked(space: *const CPSpace) -> bool;
    pub fn cpSpaceGetGravity(CPSpace: *const CPSpace) -> CPVect;
    pub fn cpSpaceGetCurrentTimeStep(space: *const CPSpace) -> CPFloat;
    pub fn cpSpaceGetIterations(space: *const CPSpace) -> c_int;
    pub fn cpSpaceGetDamping(space: *const CPSpace) -> CPFloat;
    pub fn cpSpaceGetIdleSpeedThreshold(space: *const CPSpace) -> CPFloat;
    pub fn cpSpaceGetSleepTimeThreshold(space: *const CPSpace) -> CPFloat;
    pub fn cpSpaceGetCollisionSlop(space: *const CPSpace) -> CPFloat;
    pub fn cpSpaceGetCollisionBias(space: *const CPSpace) -> CPFloat;
    pub fn cpSpaceGetCollisionPersistence(space: *const CPSpace) -> c_uint;
    pub fn cpSpaceGetStaticBody(space: *const CPSpace) -> *const CPBody;
    pub fn cpSpaceSetGravity(CPSpace: *const CPSpace, gravity: CPVect);
    pub fn cpSpaceSetIterations(space: *const CPSpace, value: c_int);
    pub fn cpSpaceSetDamping(space: *const CPSpace, value: CPFloat);
    pub fn cpSpaceSetIdleSpeedThreshold(space: *const CPSpace, value: CPFloat);
    pub fn cpSpaceSetSleepTimeThreshold(space: *const CPSpace, value: CPFloat);
    pub fn cpSpaceSetCollisionSlop(space: *const CPSpace, value: CPFloat);
    pub fn cpSpaceSetCollisionBias(space: *const CPSpace, value: CPFloat);
    pub fn cpSpaceSetCollisionPersistence(space: *const CPSpace, value: c_uint);

    // Space Operations
    pub fn cpSpaceStep(space: *const CPSpace, dt: CPFloat);
    pub fn cpSpaceAddShape(space: *const CPSpace, shape: *const CPShape) -> *const CPShape;
    pub fn cpSpaceAddBody(space: *const CPSpace, body: *const CPBody) -> *const CPBody;
    pub fn cpSpaceRemoveShape(space: *const CPSpace, shape: *const CPShape);
    pub fn cpSpaceRemoveBody(space: *const CPSpace, body: *const CPBody);
    pub fn cpSpaceContainsShape(space: *const CPSpace, shape: *const CPShape) -> bool;
    pub fn cpSpaceContainsBody(space: *const CPSpace, body: *const CPBody) -> bool;
    pub fn cpSpaceAddConstraint(space: *const CPSpace, constraint: CPConstraint);
    pub fn cpSpaceRemoveConstraint(space: *const CPSpace, constraint: CPConstraint);
    pub fn cpSpaceContainsConstraint(space: *const CPSpace, constraint: CPConstraint) -> bool;

    pub fn cpSpaceReindexShape(space: *const CPSpace, shape: *const CPShape);
    pub fn cpSpaceReindexShapesForBody(space: *const CPSpace, body: *const CPBody);
    pub fn cpSpaceReindexStatic(space: *const CPSpace);
    // TODO iterators for body/shape/constraint

    // Body
    pub fn cpBodyNew(m: CPFloat, i: CPFloat) -> *const CPBody;
    pub fn cpBodyNewStatic() -> *const CPBody;
    pub fn cpBodyNewKinematic() -> *const CPBody;
    pub fn cpBodyFree(body: *const CPBody);

    pub fn cpBodyGetMass(body: *const CPBody) -> CPFloat;
    pub fn cpBodySetMass(body: *const CPBody, m: CPFloat);
    pub fn cpBodyGetMoment(body: *const CPBody) -> CPFloat;
    pub fn cpBodySetMoment(body: *const CPBody, i: CPFloat);
    pub fn cpBodyGetPosition(body: *const CPBody) -> CPVect;
    pub fn cpBodySetPosition(body: *const CPBody, pos: CPVect);
    pub fn cpBodyGetCenterOfGravity(body: *const CPBody) -> CPVect;
    pub fn cpBodySetCenterOfGravity(body: *const CPBody, cog: CPVect);
    pub fn cpBodyGetVelocity(body: *const CPBody) -> CPVect;
    pub fn cpBodySetVelocity(body: *const CPBody, value: CPVect);
    pub fn cpBodyGetForce(body: *const CPBody) -> CPVect;
    pub fn cpBodySetForce(body: *const CPBody, value: CPVect);
    pub fn cpBodyGetAngle(body: *const CPBody) -> CPFloat;
    pub fn cpBodySetAngle(body: *const CPBody, a: CPFloat);
    pub fn cpBodyGetAngularVelocity(body: *const CPBody) -> CPFloat;
    pub fn cpBodySetAngularVelocity(body: *const CPBody, value: CPFloat);
    pub fn cpBodyGetTorque(body: *const CPBody) -> CPFloat;
    pub fn cpBodySetTorque(body: *const CPBody, value: CPFloat);
    pub fn cpBodyGetRotation(body: *const CPBody) -> CPVect;
    pub fn cpBodyGetSpace(body: *const CPBody) -> *const CPSpace; // May be null

    // Shape
    pub fn cpShapeFree(shape: *const CPShape);

    pub fn cpShapeGetSpace(shape: *const CPShape) -> *const CPSpace; // May be null
    pub fn cpShapeGetBody(shape: *const CPShape) -> *const CPBody;

    pub fn cpShapeSetBody(shape: *const CPShape, body: *const CPBody);
    pub fn cpShapeGetElasticity(shape: *const CPShape) -> CPFloat;
    pub fn cpShapeSetElasticity(shape: *const CPShape, value: CPFloat);
    pub fn cpShapeGetFriction(shape: *const CPShape) -> CPFloat;
    pub fn cpShapeSetFriction(shape: *const CPShape, value: CPFloat);
    // pub fn cpShapeGetBB(shape: *const CPShape) -> cpBB;
    // pub fn cpShapeGetSensor(shape: *const CPShape) -> cpBool;
    // pub fn cpShapeSetSensor(shape: *const CPShape, value: cpBool);
    // pub fn cpShapeGetSurfaceVelocity(shape: *const CPShape) -> CPVect;
    // pub fn cpShapeSetSurfaceVelocity(shape: *const CPShape, value: CPVect);
    // pub fn cpShapeGetCollisionType(shape: *const CPShape) -> cpCollisionType;
    // pub fn cpShapeSetCollisionType(shape: *const CPShape, value: cpCollisionType);
    // pub fn cpShapeGetFilter(shape: *const CPShape) -> cpShapeFilter;
    // pub fn cpShapeSetFilter(shape: *const CPShape, filter: cpShapeFilter);

    // Circle Shape
    pub fn cpCircleShapeNew(body: *const CPBody,
                            radius: CPFloat,
                            offset: CPVect)
                            -> *const CPCircleShape;
    pub fn cpCircleShapeGetOffset(circleShape: *const CPCircleShape) -> CPVect;
    pub fn cpCircleShapeGetRadius(circleShape: *const CPCircleShape) -> CPFloat;
}