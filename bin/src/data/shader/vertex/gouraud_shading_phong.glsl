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
    
    vec4 camera_position=inverse(view)*camera_origin;
    
    vec4 p=position_world;
    
    vec4 v=normalize(camera_position-p);
    
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
    lambert_diffuse_term=global_lighting*max(0,dot(n,l));
    
    // Vetor que define o sentido da reflexão especular ideal.
    vec4 r=-l+2*n*(dot(n,l));
    
    // Termo especular utilizando o modelo de iluminação de Phong
    phong_specular_term=global_lighting*pow(max(0,dot(r,v)),phong_q);
    
}

