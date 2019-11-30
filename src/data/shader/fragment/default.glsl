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

// Parametros de refletancia ambiente
uniform vec3 ambient_reflectance;

// Parametro de sobreescrita de cor
uniform vec3 color_overide;

// Parametro de expoente q de phong
uniform float phong_q;

// Textura map type: Tipo de mapeamento da textura. 0 - Plano; 1- Planar XY; 2- Esferico; 3- Cilindrico
uniform int texture_map_type;

// Direção da iluminação global
uniform vec4 lighting_direction;

// Possivel vetor de sobrescrita da iluminaçção global
uniform vec4 lighting_source_override;

out vec3 color;
void main()
{
    color=color_overide;
}

