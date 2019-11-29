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

// Parametros de reflexão specular
uniform vec3 specular_reflectance;

// Parametros de luz ambiente
uniform vec3 ambient_lighting;

// Parametro de sobreescrita de cor
uniform vec3 color_overide;

// Parametro de expoente q de phong
uniform float phong_q;

// Textura map type: Tipo de mapeamento da textura. 0 - ARQUIVO OBJ; 1- Planar XY; 2- Esferico; 3- Cilindrico
uniform int texture_map_type;

uniform vec4 lighting_direction;

out vec3 color;

void main()
{
    // INICIALIZAÇÂO:
    
    // sistema de coordenadas da câmera.
    vec4 camera_position=inverse(view)*camera_origin;
    
    vec4 p=position_world;
    
    // Normal do fragmento atual, interpolada pelo rasterizador a partir das
    // normais de cada vértice.
    vec4 n=normalize(normal);
    
    // Vetor que define o sentido da fonte de luz em relação ao ponto atual.
    vec4 l=normalize(lighting_direction);
    
    // Vetor que define o sentido da câmera em relação ao ponto atual.
    vec4 v=normalize(camera_position-p);
    
    // Coordenadas de textura U e V
    float U=0.;
    float V=0.;
    
    vec3 object_reflectance=color_overide;
    
    // FIM INICIALIZACAO
    
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
        }
        else{
            
            // Coordenadas de textura do plano, obtidas do arquivo OBJ.
            U=texcoords.x;
            V=texcoords.y;
            
        }
        
        object_reflectance=texture(texture_overide,vec2(U,V)).rgb;
    }
    
    // Termo difuso utilizando a lei dos cossenos de Lambert
    vec3 lambert_diffuse_term=global_lighting*max(0,dot(n,l));
    
    // Termo ambiente
    vec3 ambient_term=color_overide*ambient_lighting;
    
    color=(lambert_diffuse_term*object_reflectance);
    
    color=pow(color,vec3(1.,1.,1.)/2.2);
}

