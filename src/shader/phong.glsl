#version 330 core

in vec4 position_world;
in vec4 normal;

// Matrizes computadas no código C++ e enviadas para a GPU
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

// Variáveis para acesso das imagens de textura
uniform sampler2D TextureImage0;

// Parâmetros da axis-aligned bounding box (AABB) do modelo
uniform vec4 bbox_min;
uniform vec4 bbox_max;

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
    vec4 origin=vec4(0.,0.,0.,1.);
    vec4 camera_position=inverse(view)*origin;
    
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
    
    // Parâmetros que definem as propriedades espectrais da superfície
    // Refletância difusa
    vec3 Kd;
    // Refletância especular
    vec3 Ks;
    // Refletância ambiente
    vec3 Ka;
    // Expoente especular para o modelo de iluminação de Phong
    float q;
    
    // Propriedades espectrais do coelho
    Kd=vec3(.08,.4,.8);
    Ks=vec3(.8,.8,.8);
    Ka=vec3(.04,.2,.4);
    q=32.;
    
    // Espectro da fonte de iluminação
    vec3 I=vec3(1.,1.,1.);
    
    // Espectro da luz ambiente
    vec3 Ia=vec3(.9412,.7255,.7255);
    
    // Termo difuso utilizando a lei dos cossenos de Lambert
    vec3 lambert_diffuse_term=Kd*I*max(0,dot(n,l));
    
    // Termo ambiente
    vec3 ambient_term=Ka*Ia;
    
    // Termo especular utilizando o modelo de iluminação de Phong
    vec3 phong_specular_term=Ks*I*pow(max(0,dot(r,v)),q);
    
    color=lambert_diffuse_term+ambient_term+phong_specular_term;
    
    // Cor final com correção gamma, considerando monitor sRGB.
    // Veja https://en.wikipedia.org/w/index.php?title=Gamma_correction&oldid=751281772#Windows.2C_Mac.2C_sRGB_and_TV.2Fvideo_standard_gammas
    color=pow(color,vec3(1.,1.,1.)/2.2);
}

