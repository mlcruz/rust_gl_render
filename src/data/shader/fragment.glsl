#version 330 core

in vec4 position_world;
in vec4 normal;

// Posição do vértice atual no sistema de coordenadas local do modelo.
in vec4 position_model;

// Coordenadas de textura obtidas do arquivo OBJ (se existirem!)
in vec2 texcoords;

// Matrizes computadas no código C++ e enviadas para a GPU
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

// Variáveis para acesso das imagens de textura
uniform sampler2D texture_overide;

// Parâmetros da axis-aligned bounding box (AABB) do modelo
uniform vec4 bbox_min;
uniform vec4 bbox_max;

// Parametros de iluminação global
uniform vec3 global_lighting;

// Parametros de origem da camera
uniform vec4 camera_origin;

uniform vec3 specular_reflectance;

uniform vec3 ambient_lighting;

out vec3 color;

// Constantes
#define M_PI 3.14159265358979323846
#define M_PI_2 1.57079632679489661923

// Coordenadas de textura U e V
float U=0.;
float V=0.;

void main()
{
    
    // Obtemos a posição da câmera utilizando a inversa da matriz que define o
    // sistema de coordenadas da câmera.
    vec4 camera_position=inverse(view)*camera_origin;
    
    vec4 p=position_world;
    
    // Normal do fragmento atual, interpolada pelo rasterizador a partir das
    // normais de cada vértice.
    vec4 n=normalize(normal);
    
    // Vetor que define o sentido da fonte de luz em relação ao ponto atual.
    vec4 l=normalize(vec4(1.,1.,0.,0.));
    
    // Vetor que define o sentido da câmera em relação ao ponto atual.
    vec4 v=normalize(camera_position-p);
    
    // Vetor que define o sentido da reflexão especular ideal.
    vec4 r=-l+2*n*(dot(n,l));
    
    float q=32.;
    
    float minx=bbox_min.x;
    float maxx=bbox_max.x;
    
    float miny=bbox_min.y;
    float maxy=bbox_max.y;
    
    float minz=bbox_min.z;
    float maxz=bbox_max.z;
    
    U=(position_model.x-minx)/(maxx-minx);
    V=(position_model.y-miny)/(maxy-miny);
    
    vec3 Kd0=texture(texture_overide,vec2(U,V)).rgb;
    
    // Espectro da luz ambiente
    //vec3 Ia=vec3(.9412,.7255,.7255);
    
    // Termo difuso utilizando a lei dos cossenos de Lambert
    vec3 lambert_diffuse_term=Kd0*global_lighting*max(0,dot(n,l));
    
    // Termo ambiente
    vec3 ambient_term=Kd0*ambient_lighting;
    
    // Termo especular utilizando o modelo de iluminação de Phong
    vec3 phong_specular_term=specular_reflectance*global_lighting*pow(max(0,dot(r,v)),q);
    
    color=lambert_diffuse_term+ambient_term+phong_specular_term;
    
    // Cor final com correção gamma, considerando monitor sRGB.
    // Veja https://en.wikipedia.org/w/index.php?title=Gamma_correction&oldid=751281772#Windows.2C_Mac.2C_sRGB_and_TV.2Fvideo_standard_gammas
    color=pow(color,vec3(1.,1.,1.)/2.2);
}

