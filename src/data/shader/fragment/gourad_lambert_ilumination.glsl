#version 330 core

// Interpolação da posição normal e normal de cada vertice
in vec4 position_world;
in vec4 normal;

// Posição do vértice atual no sistema de coordenadas local do modelo.
in vec4 position_model;

// Coordenadas de textura obtidas do arquivo OBJ (se existirem!)
in vec2 texcoords;

in vec3 phong_specular_term;
in vec3 lambert_diffuse_term;

in vec3 color_v;

out vec3 color;

void main()
{
    color=pow(color_v,vec3(1.,1.,1.)/2.2);
}

