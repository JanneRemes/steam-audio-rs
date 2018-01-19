extern crate libc;

pub type IPLvoid = libc::c_void;
pub type IPLint8 = libc::c_char;
pub type IPLuint8 = libc::c_uchar;
pub type IPLint16 = libc::c_short;
pub type IPLuint16 = libc::c_ushort;
pub type IPLint32 = libc::c_int;
pub type IPLuint32 = libc::c_uint;
pub type IPLint64 = libc::c_longlong;
pub type IPLuint64 = libc::c_ulonglong;
pub type IPLfloat32 = libc::c_float;
pub type IPLfloat64 = libc::c_double;
pub type IPLbyte = libc::c_uchar;
pub type IPLsize = usize;
pub type IPLstring = *mut libc::c_char;

/// An opaque handle to a Phonon API object. A variable of this type may not be cast to a pointer to any other
/// API type.
pub type IPLhandle = *mut libc::c_void;

/// Boolean values.
c_enum! {
    typedef enum {
        IPL_FALSE,
        IPL_TRUE
    } IPLbool;
}

/// Status codes returned by Phonon API functions.
c_enum! {
    typedef enum {
        IPL_STATUS_SUCCESS,
        IPL_STATUS_FAILURE,
        IPL_STATUS_OUTOFMEMORY,
        IPL_STATUS_INITIALIZATION
    } IPLerror;
}

/// Prototype of a callback that logs a message generated by Phonon. This may be implemented in any suitable way,
/// such as appending to a log file, displaying a dialog box, etc. The default behavior is to print to \c stdout.
///
/// \param  message     The message to log.
///
pub type IPLLogFunction = extern "C" fn(*mut libc::c_char);

/// Prototype of a callback that allocates memory. This is usually specified to let Phonon use a custom memory
/// allocator. The default behavior is to use the OS-dependent aligned version of \c malloc.
///
/// \param  size        The number of bytes to allocate.
/// \param  alignment   The alignment (in bytes) of the start address of the allocated memory.
///
/// \return Pointer to the allocated block of memory, or \c NULL if allocation failed.
///
pub type IPLAllocateFunction = extern "C" fn(IPLsize, IPLsize) -> *mut IPLvoid;

/// Prototype of a callback that frees a block of memory. This is usually specified when using a custom memory
/// allocator with Phonon. The default behavior is to use the OS-dependent aligned version of \c free.
///
/// \param  memoryBlock Pointer to the block of memory.
///
pub type IPLFreeFunction = extern "C" fn(*mut IPLvoid);

///*****************************************************************************************************************/
///* Geometry                                                                                                      */
///*****************************************************************************************************************/

/// A point or vector in 3D space. Phonon uses a right-handed coordinate system, with the positive x-axis pointing
/// right, the positive y-axis pointing up, and the negative z-axis pointing ahead. Position and direction data
/// obtained from a game engine or audio engine must be properly transformed before being passed to any Phonon API
/// function.
///
c_struct! {
    typedef struct {
        IPLfloat32 : x;
        IPLfloat32 : y;
        IPLfloat32 : z;
    } IPLVector3;
}

/// A unit-length quaternion. Quaternions are used to represent a rotation or orientation.
///
c_struct! {
    typedef struct {
        IPLfloat32 : x;
        IPLfloat32 : y;
        IPLfloat32 : z;
        IPLfloat32 : w;
    } IPLQuaternion;
}

/// An axis-aligned box. Axis-aligned boxes are used to specify a volume of 3D space.
///
c_struct! {
    typedef struct {
        IPLVector3 : minCoordinates;
        IPLVector3 : maxCoordinates;
    } IPLBox;
}

/// An oriented box. Oriented boxes are used to specify a volume of 3D space.
///
c_struct! {
    typedef struct {
        IPLVector3    : mCenter;  
        IPLVector3    : mExtents; 
        IPLQuaternion : mRotation;
    } IPLOrientedBox;
}

/// A sphere. Spheres are used to define a region of influence around a point.
///
c_struct! {
    typedef struct {
        IPLVector3 : center;
        IPLfloat32 : radius;
    } IPLSphere;
}

///*****************************************************************************************************************/
///* OpenCL Compute Devices                                                                                        */
///*****************************************************************************************************************/

/// The type of device to use with OpenCL. The appropriate OpenCL drivers must be installed on the user's system.
/// Multiple OpenCL drivers may be installed on the same system; in this case the first available driver that
/// exposes the specified kind of device will be used.
///
c_enum! {
    typedef enum {
        IPL_COMPUTEDEVICE_CPU,
        IPL_COMPUTEDEVICE_GPU,
        IPL_COMPUTEDEVICE_ANY
    } IPLComputeDeviceType;
}

/// Specifies constraints on the type of OpenCL device to create. This information is intended to be passed to
/// \c iplCreateComputeDevice.
///
c_struct! {
    typedef struct {
        IPLComputeDeviceType  : dtype;               
        IPLbool               : requiresTrueAudioNext;
        IPLint32              : minReservableCUs;    
        IPLint32              : maxCUsToReserve;     
    } IPLComputeDeviceFilter;
}

///*****************************************************************************************************************/
///* Simulation Settings                                                                                           */
///*****************************************************************************************************************/

/// The ray tracer to use for scene representation and simulation. Phonon lets you choose from multiple ray
/// tracing implementations, each with different trade-offs. You can also choose to use your own ray tracing
/// implementation.
///
c_enum! {
    typedef enum {
        IPL_SCENETYPE_PHONON,
        IPL_SCENETYPE_EMBREE,  
        IPL_SCENETYPE_FIRERAYS,
        IPL_SCENETYPE_CUSTOM
    } IPLSceneType;
}

/// The type of simulation to perform. All sound sources must use the same type of simulation; it is not
/// currently possible to use real-time simulation for some sources and baked data for others.
///
c_enum! {
    typedef enum {
        IPL_SIMTYPE_REALTIME,
        IPL_SIMTYPE_BAKED    
    } IPLSimulationType;
}

/// Configures the complexity of the simulation. You can fine-tune these values to arrive at a suitable
/// balance between performance, memory usage, and acoustic detail.
///
c_struct! {
    typedef struct {
        IPLSceneType : sceneType;
        IPLint32     : numRays;
        IPLint32     : numDiffuseSamples;
        IPLint32     : numBounces;
        IPLfloat32   : irDuration;
        IPLint32     : ambisonicsOrder;
        IPLint32     : maxConvolutionSources; 
    } IPLSimulationSettings;
}

///*****************************************************************************************************************/
///* Scene                                                                                                         */
///*****************************************************************************************************************/

/// A triangle in 3D space. Triangles are specified by their three vertices, which are in turn specified using
/// indices into a vertex array. See iplSetStaticMeshVertices for how to specify the vertex array. Phonon uses
/// a counter-clockwise winding order. This means that when looking at the triangle such that the normal is
/// pointing towards you, the vertices are specified in counter-clockwise order.
///
c_struct! {
    typedef struct {
        [IPLint32; 3] : indices;
    } IPLTriangle;
}

/// The acoustic properties of a surface. You can specify the acoustic material properties of each triangle,
/// although typically many triangles will share a common material. The acoustic material properties are specified
/// for three frequency bands with center frequencies of 400 Hz, 2.5 KHz, and 15 KHz.
///
/// Below are the acoustic material properties for a few standard materials.
///
/// ```cpp
/// {"generic",{0.10f,0.20f,0.30f,0.05f,0.100f,0.050f,0.030f}}
/// {"brick",{0.03f,0.04f,0.07f,0.05f,0.015f,0.015f,0.015f}}
/// {"concrete",{0.05f,0.07f,0.08f,0.05f,0.015f,0.002f,0.001f}}
/// {"ceramic",{0.01f,0.02f,0.02f,0.05f,0.060f,0.044f,0.011f}}
/// {"gravel",{0.60f,0.70f,0.80f,0.05f,0.031f,0.012f,0.008f}},
/// {"carpet",{0.24f,0.69f,0.73f,0.05f,0.020f,0.005f,0.003f}}
/// {"glass",{0.06f,0.03f,0.02f,0.05f,0.060f,0.044f,0.011f}}
/// {"plaster",{0.12f,0.06f,0.04f,0.05f,0.056f,0.056f,0.004f}}
/// {"wood",{0.11f,0.07f,0.06f,0.05f,0.070f,0.014f,0.005f}}
/// {"metal",{0.20f,0.07f,0.06f,0.05f,0.200f,0.025f,0.010f}}
/// {"rock",{0.13f,0.20f,0.24f,0.05f,0.015f,0.002f,0.001f}}
/// ```
///
c_struct! {
    typedef struct {
        IPLfloat32 : lowFreqAbsorption;
        IPLfloat32 : midFreqAbsorption;
        IPLfloat32 : highFreqAbsorption;
        IPLfloat32 : scattering;
        IPLfloat32 : lowFreqTransmission;
        IPLfloat32 : midFreqTransmission;
        IPLfloat32 : highFreqTransmission;
    } IPLMaterial;
}

/// A callback that is called to update the application on the progress of the iplLoadScene function. You can
/// use this to provide the user with visual feedback, like a progress bar.
///
/// \param  progress    Fraction of the loading process that has been completed, between 0.0 and 1.0.
///
pub type IPLLoadSceneProgressCallback = extern "C" fn(progress: IPLfloat32);

/// A callback that is called to update the application on the progress of the iplFinalizeScene function. You can
/// use this to provide the user with visual feedback, like a progress bar.
///
/// \param  progress    Fraction of the finalization process that has been completed, between 0.0 and 1.0.
///
pub type IPLFinalizeSceneProgressCallback = extern "C" fn(progress: IPLfloat32);

/// A callback that is called to calculate the closest hit along a ray. Strictly speaking, the intersection is
/// calculated with a ray _interval_ (equivalent to a line segment). Any ray interval may have multiple points
/// of intersection with scene geometry; this function must return information about the point of intersection that
/// is closest to the ray's origin.
///
/// \param  origin              Array containing the x, y, z coordinates (in that order) of the ray's origin.
/// \param  direction           Array containing the x, y, z coordinates (in that order) of a unit-length vector
///                             along the ray's direction.
/// \param  minDistance         The minimum distance from the origin at which an intersection may occur for it
///                             to be considered. This function must not return any intersections closer to the
///                             origin than this value.
/// \param  maxDistance         The maximum distance from the origin at which an intersection may occur for it
///                             to be considered. This function must not return any intersections farther from
///                             the origin than this value.
/// \param  hitDistance         [out] Distance between the origin and the closest intersection point on the ray.
/// \param  hitNormal           [out] Array containing the x, y, z coordinates (in that order) of the unit-length
///                             surface normal of the geometry at the closest intersection point.
/// \param  hitMaterial         [out] Address of a pointer to the material properties of the surface at the closest
///                             intersection point. The array contains the low-, mid-, and high-frequency
///                             absorption coefficients, the scattering coefficient, and the low-, mid-, and
///                             high-frequency transmission coefficients, in that order.
/// \param  userData            Pointer a block of memory containing arbitrary data, specified during the call to
///                             \c ::iplSetRayTracerCallbacks.
///
pub type IPLClosestHitCallback = extern "C" fn(origin: *const IPLfloat32,
                                               direction: *const IPLfloat32,
                                               minDistance: IPLfloat32,
                                               maxDistance: IPLfloat32,
                                               hitDistance: *mut IPLfloat32,
                                               hitNormal: *mut IPLfloat32,
                                               hitMaterial: *mut *mut IPLMaterial,
                                               userData: *mut IPLvoid);

/// A callback that is called to calculate whether a ray hits any geometry. Strictly speaking, the function
/// looks for any intersection with a ray _interval_ (equivalent to a line segment).
///
/// \param  origin              Array containing the x, y, z coordinates (in that order) of the ray's origin.
/// \param  direction           Array containing the x, y, z coordinates (in that order) of a unit-length vector
///                             along the ray's direction.
/// \param  minDistance         The minimum distance from the origin at which an intersection may occur for it
///                             to be considered.
/// \param  maxDistance         The maximum distance from the origin at which an intersection may occur for it
///                             to be considered.
/// \param  hitExists           [out] An integer indicating whether the ray intersects any geometry. A value of 0
///                             indicates no intersection, 1 indicates that an intersection exists.
/// \param  userData            Pointer a block of memory containing arbitrary data, specified during the call to
///                             \c ::iplSetRayTracerCallbacks.
///
pub type IPLAnyHitCallback = extern "C" fn(origin: *const IPLfloat32,
                                           direction: *const IPLfloat32,
                                           minDistance: IPLfloat32,
                                           maxDistance: IPLfloat32,
                                           hitExists: *mut IPLint32,
                                           userData: *mut IPLvoid);

///*****************************************************************************************************************/
///* Rendering Settings                                                                                            */
///*****************************************************************************************************************/

/// The backend to use for applying convolution effects for sound propagation. Phonon lets you choose from
/// multiple convolution implementations, with different trade-offs.
///
c_enum! {
    typedef enum {
        IPL_CONVOLUTIONTYPE_PHONON,
        IPL_CONVOLUTIONTYPE_TRUEAUDIONEXT
    } IPLConvolutionType;
}

/// Describes various properties of the audio processing pipeline. Many Phonon API objects that are used by the
/// audio engine need to know how the audio processing pipeline (i.e., your audio engine) applies DSP effects to
/// audio data. This structure describes the key parameters.
///
c_struct! {
    typedef struct {
        IPLint32           : samplingRate;
        IPLint32           : frameSize;
        IPLConvolutionType : convolutionType;
    } IPLRenderingSettings;
}

///*****************************************************************************************************************/
///* Audio Buffers                                                                                                 */
///*****************************************************************************************************************/

/// Whether the audio buffer is encoded using Ambisonics or not.
///
c_enum! {
    typedef enum {
        IPL_CHANNELLAYOUTTYPE_SPEAKERS,
        IPL_CHANNELLAYOUTTYPE_AMBISONICS
    } IPLChannelLayoutType;
}

/// The type of speaker configuration, for audio formats that are not encoded using Ambisonics.
///
c_enum! {
    typedef enum {
        IPL_CHANNELLAYOUT_MONO,
        IPL_CHANNELLAYOUT_STEREO,
        IPL_CHANNELLAYOUT_QUADRAPHONIC,
        IPL_CHANNELLAYOUT_FIVEPOINTONE,
        IPL_CHANNELLAYOUT_SEVENPOINTONE,
        IPL_CHANNELLAYOUT_CUSTOM
    } IPLChannelLayout;
}

/// The order in which Ambisonics channels are stored in an audio buffer. Each Ambisonics channel is a series of
/// coefficients for a corresponding basis function, denoted by \f$ Y_l^m(\theta,\phi) \f$, where \f$\theta\f$ and
/// \f$\phi\f$ are two angles which pinpoint the source relative to the listener, and \f$l\f$ and \f$m\f$ are two
/// two integers which, taken together, identify a single Ambisonics channel. Here, \f$ l \geq 0 \f$ and
/// \f$ -l \leq m \leq l \f$.
///
/// There are many different conventions used by the audio engineering community to encode Ambisonics coefficients.
/// Phonon supports many of them.
///
/// This enumeration defines the sequence in which Ambisonics channels are stored. Since two integers are needed to
/// identify an Ambisonics channel, there is more than one way to use a single integer to identify an Ambisonics
/// channel.
///
c_enum! {
    typedef enum {
        IPL_AMBISONICSORDERING_FURSEMALHAM,
        IPL_AMBISONICSORDERING_ACN
    } IPLAmbisonicsOrdering;
}

/// Normalization conventions for Ambisonics channels. There are a few different ways of normalizing the values of
/// the Ambisonics channels relative to each other. Phonon supports the most popular ones.
///
c_enum! {
    typedef enum {
        IPL_AMBISONICSNORMALIZATION_FURSEMALHAM,
        IPL_AMBISONICSNORMALIZATION_SN3D,
        IPL_AMBISONICSNORMALIZATION_N3D
    } IPLAmbisonicsNormalization;
}

/// Whether the data is interleaved or deinterleaved.
///
c_enum! {
    typedef enum {
        IPL_CHANNELORDER_INTERLEAVED,
        IPL_CHANNELORDER_DEINTERLEAVED
    } IPLChannelOrder;
}

/// The format of an audio buffer. Whenever you pass audio data to or from Phonon, you must describe the format in
/// which the audio is encoded. **Phonon only supports uncompressed PCM wave data, stored in 32-bit floating point
/// format**. However, Phonon supports many different multi-channel and Ambisonics formats, and the
/// \c IPLAudioFormat tells Phonon how to interpret a buffer of audio data.
///
c_struct! {
    typedef struct {
        IPLChannelLayoutType       : channelLayoutType;          
        IPLChannelLayout           : channelLayout;              
        IPLint32                   : numSpeakers;                
        *mut IPLVector3            : speakerDirections;          
        IPLint32                   : ambisonicsOrder;            
        IPLAmbisonicsOrdering      : ambisonicsOrdering;         
        IPLAmbisonicsNormalization : ambisonicsNormalization;    
        IPLChannelOrder            : channelOrder;               
    } IPLAudioFormat;
}

/// A buffer containing audio data. All audio data passed to or from Phonon must be packaged in \c IPLAudioBuffer
/// objects, which describe the format and size of the audio data.
///
c_struct! {
    typedef struct {
        IPLAudioFormat           : format;
        IPLint32                 : numSamples;
        *mut IPLfloat32          : interleavedBuffer;
        *mut *mut IPLfloat32     : deinterleavedBuffer;
    } IPLAudioBuffer;
}

///*****************************************************************************************************************/
///* Binaural Renderer                                                                                             */
///*****************************************************************************************************************/

/// The type of HRTF database to use for binaural rendering. You can either use the built-in HRTF database, or
/// supply your own HRTF data at run-time.
///
c_enum! {
    typedef enum {
        IPL_HRTFDATABASETYPE_DEFAULT,
        IPL_HRTFDATABASETYPE_CUSTOM
    } IPLHrtfDatabaseType;
}

/// A single-precision complex number.
///
c_struct! {
    typedef struct {
        IPLfloat32 : real;
        IPLfloat32 : imag;
    } IPLComplex;
}

/// A function that you can call to calculate the Fast Fourier Transform (FFT) of a real-valued time-domain
/// signal. You will typically call this from within your implementation of IPLHrtfLoadCallback, to transform your
/// time-domain Head-Related Impulse Responses (HRIRs) into Head-Related Transfer Functions (HRTFs).
///
/// \param  data                Pointer to internal data required for calculating Fourier transforms. This will be
///                             passed in to your implementation of IPLHrtfLoadCallback.
/// \param  signal              Array containing the time-domain signal. The number of elements in this array must
///                             match the signalSize parameter received by IPLHrtfLoadCallback.
/// \param  spectrum            Array containing the frequency-domain spectrum. The number of elements in this
///                             array must match the spectrumSize parameter received by IPLHrtfLoadCallback.
///
pub type IPLFftHelper = extern "C" fn(data: *mut IPLvoid,
                                      signal: *mut IPLfloat32,
                                      spectrum: *mut IPLComplex);

/// Pointer to a function that will be called during the execution of iplCreateBinauralRenderer, to allow your
/// application to pre-transform all HRTF data into frequency domain.
///
/// \param  signalSize          Number of elements in the time-domain (HRIR) data arrays that must be transformed.
///                             This will be greater than the actual size of the HRIRs passed to
///                             iplCreateBinauralRenderer. Any array passed to fftHelper must contain the HRIR
///                             data at the start, and the rest of the elements must be initialized to zero. For
///                             example, if signalSize is 1024, and the HRIRs are 200 samples long, the arrays
///                             passed to the signal parameter of fftHelper must be 1024 samples long, with the
///                             first 200 samples containing the HRIR data, and the remaining 824 samples containing
///                             zeroes.
/// \param  spectrumSize        Number of elements in the frequency-domain (HRTF) data arrays that will contain the
///                             results of the transformation. You will typically allocate arrays of this size for
///                             each HRIR; they must not be freed until IPLHrtfUnloadCallback is called.
/// \param  fftHelper           Pointer to a function that you can call to calculate the Fourier transforms of the
///                             HRIRs.
/// \param  fftHelperData       Internal data required for calculating Fourier transforms. Pass this to fftHelper.
///
pub type IPLHrtfLoadCallback = extern "C" fn(signalSize: IPLint32,
                                             spectrumSize: IPLint32,
                                             fftHelper: IPLFftHelper,
                                             fftHelperData: *mut IPLvoid);

/// Pointer to a function that will be called during the execution of iplDestroyBinauralRenderer, to allow your
/// application to free memory allocated during IPLHrtfLoadCallback.
///
pub type IPLHrtfUnloadCallback = extern "C" fn();

/// Pointer to a function that will be called during the execution of iplApplyBinauralEffect, to left your
/// application copy HRTF data for a given direction into arrays managed by Phonon.
///
/// \param  direction           Array containing the coordinates of the unit vector from the listener to the
///                             source, in Cartesian coordinates.
/// \param  leftHrtf            Array into which you should copy the frequency-domain left-ear HRTF for the given
///                             direction.
/// \param  rightHrtf           Array into which you should copy the frequency-domain right-ear HRTF for the given
///                             direction.
///
pub type IPLHrtfLookupCallback = extern "C" fn(direction: *mut IPLfloat32,
                                               leftHrtf: *mut IPLComplex,
                                               rightHrtf: *mut IPLComplex);

/// Parameters used to describe the HRTF database you want to use when creating a Binaural Renderer object.
///
c_struct! {
    typedef struct {
       IPLHrtfDatabaseType           : dtype;          
       *mut IPLbyte                  : hrtfData;      
       IPLint32                      : numHrirSamples;
       IPLHrtfLoadCallback           : loadCallback;  
       IPLHrtfUnloadCallback         : unloadCallback;
       IPLHrtfLookupCallback         : lookupCallback;
    } IPLHrtfParams;
}

///*****************************************************************************************************************/
///* Object-Based Binaural Effect                                                                                  */
///*****************************************************************************************************************/

/// Techniques for interpolating HRTF data. This is used when rendering a point source whose position relative to
/// the listener is not contained in the measured HRTF data used by Phonon.
///
c_enum! {
    typedef enum {
        IPL_HRTFINTERPOLATION_NEAREST,
        IPL_HRTFINTERPOLATION_BILINEAR
    } IPLHrtfInterpolation;
}

///*****************************************************************************************************************/
///* Environmental Renderer                                                                                        */
///*****************************************************************************************************************/

/// Callback function that is called when the simulation thread is created.
///
pub type IPLSimulationThreadCreateCallback = extern "C" fn();

/// Callback function that is called when the simulation thread is destroyed.
///
pub type IPLSimulationThreadDestroyCallback = extern "C" fn();


///*****************************************************************************************************************/
///* Direct Sound                                                                                                  */
///*****************************************************************************************************************/

/// The algorithm to use when checking for direct path occlusion. Phonon can check whether a direct sound path is
/// occluded by scene geometry, and optionally how much of a sound source is occluded.
///
c_enum! {
    typedef enum {
        IPL_DIRECTOCCLUSION_RAYCAST,
        IPL_DIRECTOCCLUSION_VOLUMETRIC
    } IPLDirectOcclusionMethod;
}

/// The method to use when rendering occluded or partially occluded sound. Phonon can model sound passing through
/// solid objects, and optionally apply frequency-dependent transmission filters.
///
c_enum! {
    typedef enum {
        IPL_DIRECTOCCLUSION_NONE,                   
        IPL_DIRECTOCCLUSION_NOTRANSMISSION,         
        IPL_DIRECTOCCLUSION_TRANSMISSIONBYVOLUME,   
        IPL_DIRECTOCCLUSION_TRANSMISSIONBYFREQUENCY
    } IPLDirectOcclusionMode;
}

/// Parameters describing a direct sound path. For each frequency band, the attenuation factor applied to the
/// direct sound path is:
///
/// distanceAttenuation * airAbsorption * (occlusionFactor + (1 - occlusionFactor) * transmissionFactor)
///
c_struct! {
    typedef struct {
        IPLVector3      : direction;            
        IPLfloat32      : distanceAttenuation;  
        [IPLfloat32; 3] : airAbsorption;     
        IPLfloat32      : propagationDelay;     
        IPLfloat32      : occlusionFactor;      
        [IPLfloat32; 3] : transmissionFactor;
    } IPLDirectSoundPath;
}

/*****************************************************************************************************************/
/* Direct Sound Effect                                                                                           */
/*****************************************************************************************************************/

/// Flags that specify which parameters from \c IPLDirectSoundPath should be applied by the Direct Sound Effect.
///
c_struct! {
    typedef struct {
        IPLbool                : applyDistanceAttenuation; 
        IPLbool                : applyAirAbsorption;       
        IPLDirectOcclusionMode : directOcclusionMode;      
    } IPLDirectSoundEffectOptions;
}

///*****************************************************************************************************************/
///* Convolution Effect                                                                                            */
///*****************************************************************************************************************/

/// Defines how a set of baked data should be interpreted.
///
c_enum! {
    typedef enum {
        IPL_BAKEDDATATYPE_STATICSOURCE,   
        IPL_BAKEDDATATYPE_STATICLISTENER, 
        IPL_BAKEDDATATYPE_REVERB          
    } IPLBakedDataType;
}

/// Identifies a set of baked data. It is the application's responsibility to ensure that this data is unique
/// across the lifetime of an Environment object.
///
c_struct! {
    typedef struct {
        IPLint32          : identifier;
        IPLBakedDataType  : dtype;     
    } IPLBakedDataIdentifier;
}

///*****************************************************************************************************************/
///* Acoustic Probes                                                                                               */
///*****************************************************************************************************************/

/// The algorithm to use when generating a set of probes. Probes are generated by specifying a bounding box for a
/// portion of the scene, and an algorithm for filling the volume of the box with probes. You can generate probes
/// using different algorithms in different portions of a scene. The bounding boxes used for probe generation in
/// different regions may overlap, although this is not typical.
///
c_enum! {
    typedef enum {
        IPL_PLACEMENT_CENTROID,    
        IPL_PLACEMENT_OCTREE,      
        IPL_PLACEMENT_UNIFORMFLOOR 
    } IPLProbePlacement;
}

/// Parameters that specify how probes should be created by \c ::iplCreateProbeBox. */
c_struct! {
    typedef struct {
        IPLProbePlacement : placement;          
        IPLfloat32        : spacing;            
        IPLfloat32        : heightAboveFloor;   
        IPLint32          : maxOctreeTriangles; 
        IPLint32          : maxOctreeDepth;     
    } IPLProbePlacementParams;
}

/// A callback that is called to update the application on the progress of the \c ::iplCreateProbeBox function.
/// You can use this to provide visual feedback to the user, like a progress bar.
///
/// \param  progress            Fraction of the probe generation process that has been completed, between
///                             0.0 and 1.0.
///
pub type IPLProbePlacementProgressCallback = extern "C" fn(progress: IPLfloat32);

///*****************************************************************************************************************/
///* Baking                                                                                                        */
///*****************************************************************************************************************/

/// Specifies the kind of acoustic responses to save in the baked data.
///
c_struct! {
    typedef struct {
        IPLbool : bakeParametric;
        IPLbool : bakeConvolution;
    } IPLBakingSettings;
}