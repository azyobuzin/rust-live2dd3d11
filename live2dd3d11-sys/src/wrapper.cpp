#define L2D_TARGET_D3D11
#include "Live2D.h"
#include "Live2DModelD3D11.h"

using namespace live2d;

typedef int w_bool;

extern "C" {
    void deleteLDObject(LDObject *obj) {
        delete obj;
    }
}

// Live2D
extern "C" {
    int Live2D_getClippingMaskBufferSize() {
        return Live2D::getClippingMaskBufferSize();
    }

    void Live2D_setClippingMaskBufferSize(int size) {
        Live2D::setClippingMaskBufferSize(size);
    }

    void Live2D_init() {
        Live2D::init();
    }

    void Live2D_dispose() {
        Live2D::dispose();
    }

    const char *Live2D_getVersionStr() {
        return Live2D::getVersionStr();
    }

    l2d_uint32 Live2D_getVersionNo() {
        return Live2D::getVersionNo();
    }

    w_bool Live2D_getBuildOption_RANGE_CHECK_POINT() {
        return Live2D::getBuildOption_RANGE_CHECK_POINT();
    }

    w_bool Live2D_getBuildOption_AVATAR_OPTION_A() {
        return Live2D::getBuildOption_AVATAR_OPTION_A();
    }

    void Live2D_setVertexDoubleBufferEnabled(w_bool enabled) {
        Live2D::setVertexDoubleBufferEnabled(enabled);
    }

    w_bool Live2D_isVertexDoubleBufferEnabled() {
        return Live2D::isVertexDoubleBufferEnabled();
    }

    void Live2D_setError(l2d_uint32 errorNo) {
        Live2D::setError(errorNo);
    }

    l2d_uint32 Live2D_getError() {
        return Live2D::getError();
    }
}

// ALive2DModel
#define self static_cast<ALive2DModel*>(p)

extern "C" {
    float ALive2DModel_getParamFloat(void *p, const char *paramID) {
        return self->getParamFloat(paramID);
    }

    void ALive2DModel_setParamFloat(void *p, const char *paramID, float value, float weight) {
        self->setParamFloat(paramID, value, weight);
    }

    void ALive2DModel_addToParamFloat(void *p, const char *paramID, float value, float weight) {
        self->addToParamFloat(paramID, value, weight);
    }

    void ALive2DModel_multParamFloat(void *p, const char *paramID, float mult, float weight) {
        self->multParamFloat(paramID, mult, weight);
    }

    // TODO: inline members

    void ALive2DModel_loadParam(void *p) {
        self->loadParam();
    }

    void ALive2DModel_saveParam(void *p) {
        self->saveParam();
    }

    void ALive2DModel_init(void *p) {
        self->init();
    }

    void ALive2DModel_update(void *p) {
        self->update();
    }

    void ALive2DModel_draw(void *p) {
        self->draw();
    }

    void ALive2DModel_setPartsOpacityByID(void *p, const char *partsID, float opacity) {
        self->setPartsOpacity(partsID, opacity);
    }

    void ALive2DModel_setPartsOpacityByIndex(void *p, int partsIndex, float opacity) {
        self->setPartsOpacity(partsIndex, opacity);
    }

    float ALive2DModel_getPartsOpacityByID(void *p, const char *partsID) {
        return self->getPartsOpacity(partsID);
    }

    float ALive2DModel_getPartsOpacityByIndex(void *p, int partsIndex) {
        return self->getPartsOpacity(partsIndex);
    }

    void ALive2DModel_setupPartsOpacityGroup_alphaImpl(void *p, const char *paramGroup[], float deltaTimeSec) {
        self->setupPartsOpacityGroup_alphaImpl(paramGroup, deltaTimeSec);
    }

    void ALive2DModel_setModelImpl(void *p, ModelImpl *m) {
        self->setModelImpl(m);
    }

    ModelImpl* ALive2DModel_getModelImpl(void *p) {
        return self->getModelImpl();
    }

    ModelContext* ALive2DModel_getModelContext(void *p) {
        return self->getModelContext();
    }

    int ALive2DModel_getErrorFlags(void *p) {
        return self->getErrorFlags();
    }

    int ALive2DModel_generateModelTextureNo(void *p) {
        return self->generateModelTextureNo();
    }

    void ALive2DModel_releaseModelTextureNo(void *p, int no) {
        self->releaseModelTextureNo(no);
    }

    float ALive2DModel_getCanvasWidth(void *p) {
        return self->getCanvasWidth();
    }

    float ALive2DModel_getCanvasHeight(void *p) {
        return self->getCanvasHeight();
    }

    DrawParam* ALive2DModel_getDrawParam(void *p) {
        return self->getDrawParam();
    }

    int ALive2DModel_getDrawDataIndex(void *p, const char *drawDataID) {
        return self->getDrawDataIndex(drawDataID);
    }

    IDrawData* ALive2DModel_getDrawData(void *p, int drawIndex) {
        return self->getDrawData(drawIndex);
    }

    l2d_pointf* ALive2DModel_getTransformedPoints(void *p, int drawIndex, int *pointCount) {
        return self->getTransformedPoints(drawIndex, pointCount);
    }

    l2d_index* ALive2DModel_getIndexArray(void *p, int drawIndex, int *polygonCount) {
        return self->getIndexArray(drawIndex, polygonCount);
    }

    void ALive2DModel_updateZBuffer_TestImpl(void *p, float startZ, float stepZ) {
        self->updateZBuffer_TestImpl(startZ, stepZ);
    }

    // TODO: inline members

    void ALive2DModel_setPremultipliedAlpha(void *p, w_bool b) {
        self->setPremultipliedAlpha(b);
    }

    w_bool ALive2DModel_isPremultipliedAlpha(void *p) {
        return self->isPremultipliedAlpha();
    }

    void ALive2DModel_setAnisotropy(void *p, int n) {
        self->setAnisotropy(n);
    }

    int ALive2DModel_getAnisotropy(void *p) {
        return self->getAnisotropy();
    }
}

// Live2DModelD3D11
#define self static_cast<Live2DModelD3D11*>(p)

extern "C" {
    void Live2DModelD3D11_setGraphicsContext(ID3D11Device *device, ID3D11DeviceContext *context) {
        Live2DModelD3D11::setGraphicsContext(device, context);
    }

    void Live2DModelD3D11_deviceLostCommon() {
        Live2DModelD3D11::deviceLostCommon();
    }

    void Live2DModelD3D11_setTexture(void *p, int textureNo, ID3D11ShaderResourceView *texture) {
        self->setTexture(textureNo, texture);
    }

    void Live2DModelD3D11_deleteTextures(void *p) {
        self->deleteTextures();
    }

    Live2DModelD3D11* Live2DModelD3D11_loadModelFromFile(const char *filepath) {
        LDString pathStr(filepath);
        return Live2DModelD3D11::loadModel(pathStr);
    }

    Live2DModelD3D11* Live2DModelD3D11_loadModelFromBuffer(const void* buf, int bufSize) {
        return Live2DModelD3D11::loadModel(buf, bufSize);
    }

    void Live2DModelD3D11_deviceLostD3D(void *p) {
        self->deviceLostD3D();
    }

    void Live2DModelD3D11_setMatrix(void *p, float *matrix) {
        self->setMatrix(matrix);
    }

    void Live2DModelD3D11_setTextureColor(void *p, int textureNo, float r, float g, float b) {
        self->setTextureColor(textureNo, r, g, b);
    }

    void Live2DModelD3D11_setTextureBlendMode(void *p, int textureNo, int mode) {
        self->setTextureBlendMode(textureNo, mode);
    }

    void Live2DModelD3D11_setTextureInterpolate(void *p, int textureNo, float interpolate) {
        self->setTextureInterpolate(textureNo, interpolate);
    }

    int Live2DModelD3D11_getErrorD3D_tmp(void *p) {
        return self->getErrorD3D_tmp();
    }
}
