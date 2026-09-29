fn {{self}}(base:u32,ctx:GeoMaterialContext,sample:ptr<function,MaterialSample>,rng:ptr<function,u32>) {
(*sample).direction-=2*dot((*sample).direction,ctx.normal)*ctx.normal;
}
