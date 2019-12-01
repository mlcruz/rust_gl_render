#version 330 core

layout(location=0)in vec4 model_coefficients;
layout(location=1)in vec4 normal_coefficients;
layout(location=2)in vec2 texture_coefficients;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec4 position_world;
out vec4 position_model;

out vec4 normal;
out vec2 texcoords;

out vec3 phong_specular_term;
out vec3 lambert_diffuse_term;

// Variáveis para acesso das imagens de textura
uniform sampler2D texture_overide;

// Parâmetros da axis-aligned bounding box (AABB) do modelo
uniform vec4 bbox_min;
uniform vec4 bbox_max;

// Parametros de iluminação global
uniform vec3 global_lighting;

// Parametros de origem da camera
uniform vec4 camera_origin;

// Parametros de refletancia specular
uniform vec3 specular_reflectance;

// Parametros de refletancia ambiente
uniform vec3 ambient_reflectance;

// Parametros de luz ambiente
uniform vec3 ambient_lighting;

// Parametro de sobreescrita de cor
uniform vec3 color_overide;

// Parametro de expoente q de phong
uniform float phong_q;

// Textura map type: Tipo de mapeamento da textura. 0 - ARQUIVO OBJ; 1- Planar XY;2- Planar XZ; ; 3- Esferico; 4- Cilindrico
uniform int texture_map_type;

// Direção da iluminação global
uniform vec4 lighting_direction;

// Possivel vetor de sobrescrita da iluminaçção global
uniform vec4 lighting_source_override;

out vec3 color_v;

// Constantes
#define M_PI 3.14159265358979323846
#define M_PI_2 1.57079632679489661923

void main()
{
    
    gl_Position=projection*view*model*model_coefficients;
    
    position_world=model*model_coefficients;
    
    position_model=model_coefficients;
    
    normal=inverse(transpose(model))*normal_coefficients;
    normal.w=0.;
    texcoords=texture_coefficients;
    
    // Normal do vertice atual, interpolada pelo rasterizador a partir das
    vec4 n=normalize(normal);
    
    // Vetor que define o sentido da fonte de luz em relação ao ponto atual.
    vec4 l=vec4(0.,0.,0.,0.);
    
    // Sobreescreve iluminação global com direção relatica a alguma fonte de luz, se existir parametro
    if(lighting_source_override.y==0.){
        l=normalize(lighting_direction);
        
    }
    else{
        vec4 source_point=lighting_source_override-position_world;
        l=normalize(source_point);
    }
    
    // Termo difuso utilizando a lei dos cossenos de Lambert
    vec3 lambert_diffuse_term=global_lighting*max(0,dot(n,l));
    
    // Coordenadas de textura U e V
    float U=0.;
    float V=0.;
    
    vec3 object_reflectance=color_overide;
    
    // Se não exite cor para sobreescrever textura atual, utiliza textura
    if(color_overide==vec3(0.,0.,0.)){
        if(texture_map_type==1){
            
            // Mapeia textura de maneira planar em xy
            float minx=bbox_min.x;
            float maxx=bbox_max.x;
            
            float miny=bbox_min.y;
            float maxy=bbox_max.y;
            
            float minz=bbox_min.z;
            float maxz=bbox_max.z;
            
            U=(position_model.x-minx)/(maxx-minx);
            V=(position_model.y-miny)/(maxy-miny);
        }else if(texture_map_type==2){
            
            // Mapeia textura de maneira planar em zx
            float minx=bbox_min.x;
            float maxx=bbox_max.x;
            
            float miny=bbox_min.y;
            float maxy=bbox_max.y;
            
            float minz=bbox_min.z;
            float maxz=bbox_max.z;
            
            U=(position_model.x-minx)/(maxx-minx);
            V=(position_model.z-minz)/(maxz-minz);
        }
        else if(texture_map_type==3){
            
            vec4 bbox_center=(bbox_min+bbox_max)/2.;
            float radius=length(bbox_max.x-bbox_center.x);
            
            float theta=atan(position_model.x,position_model.z);
            float phi=asin(position_model.y/radius);
            
            U=(theta+M_PI)/(2*M_PI);
            V=(phi+M_PI_2)/M_PI;
        }
        else if(texture_map_type==4){
            
            float theta=atan(position_model.x,position_model.z);
            U=(theta+M_PI)/(2*M_PI);
            V=(position_model.y-bbox_min.y)/(bbox_max.y-bbox_min.y);
            
        }
        else{
            
            // Coordenadas de textura do plano, obtidas do arquivo OBJ.
            U=texcoords.x;
            V=texcoords.y;
        }
        
        object_reflectance=texture(texture_overide,vec2(U,V)).rgb;
    }
    
    // Termo de refletancia ambiente calculado a partir da cor das texturas
    vec3 final_ambient_reflectance=vec3((object_reflectance.x*.15)+.05,(object_reflectance.y*.15)+.05,(object_reflectance.z*.15)+.05);
    
    // Sobreescreve refletancia ambiente se existe alguma definida, se não utiliza cor do ponto para calcular
    if(ambient_reflectance!=vec3(0.,0.,0.)){
        final_ambient_reflectance=ambient_reflectance;
    }
    
    // Termo ambiente
    vec3 ambient_term=final_ambient_reflectance*ambient_lighting;
    
    // Multiplicamos o vetor de refletancia especular pela cor da textura
    // Utilizamos um vetor (specular_reflectance) para controlar a intensidade da refletancia especular do objeto
    color_v=(lambert_diffuse_term*object_reflectance)+ambient_term;
    
}
