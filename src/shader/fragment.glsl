#version 330 core

in vec4 position_world;
in vec4 normal;

// Matrizes computadas no código C++ e enviadas para a GPU
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec3 color;

void main()
{
    
    // Obtemos a posição da câmera utilizando a inversa da matriz que define o
    // sistema de coordenadas da câmera.
    vec4 origin=vec4(0.,0.,0.,1.);
    vec4 camera_position=inverse(view)*origin;
    
    // O fragmento atual é coberto por um ponto que percente à superfície de um
    // dos objetos virtuais da cena. Este ponto, p, possui uma posição no
    // sistema de coordenadas global (World coordinates). Esta posição é obtida
    // através da interpolação, feita pelo rasterizador, da posição de cada
    // vértice.
    vec4 p=position_world;
    
    // Normal do fragmento atual, interpolada pelo rasterizador a partir das
    // normais de cada vértice.
    vec4 n=normalize(normal);
    
    // Vetor que define o sentido da fonte de luz em relação ao ponto atual.
    vec4 l=normalize(vec4(1.,1.,.5,0.));
    
    // Vetor que define o sentido da câmera em relação ao ponto atual.
    vec4 v=normalize(camera_position-p);
    
    // Vetor que define o sentido da reflexão especular ideal.
    vec4 r=-l+2*n*(dot(n,l));// PREENCHA AQUI o vetor de reflexão especular ideal
    
    // Parâmetros que definem as propriedades espectrais da superfície
    vec3 Kd;// Refletância difusa
    vec3 Ks;// Refletância especular
    vec3 Ka;// Refletância ambiente
    float q;// Expoente especular para o modelo de iluminação de Phong
    
    // Propriedades espectrais do coelho
    Kd=vec3(.08,.4,.8);
    Ks=vec3(.8,.8,.8);
    Ka=vec3(.04,.2,.4);
    q=32.;
    
    // Espectro da fonte de iluminação
    vec3 I=vec3(1.,1.,1.);// PREENCH AQUI o espectro da fonte de luz
    
    // Espectro da luz ambiente
    vec3 Ia=vec3(.2,.2,.2);// PREENCHA AQUI o espectro da luz ambiente
    
    // Termo difuso utilizando a lei dos cossenos de Lambert
    vec3 lambert_diffuse_term=Kd*I*max(0,dot(n,l));// PREENCHA AQUI o termo difuso de Lambert
    
    // Termo ambiente
    vec3 ambient_term=Ka*Ia;// PREENCHA AQUI o termo ambiente
    
    // Termo especular utilizando o modelo de iluminação de Phong
    vec3 phong_specular_term=Ks*I*pow(max(0,dot(r,v)),q);// PREENCH AQUI o termo especular de Phong
    
    color=lambert_diffuse_term+ambient_term+phong_specular_term;
    
    // Cor final com correção gamma, considerando monitor sRGB.
    // Veja https://en.wikipedia.org/w/index.php?title=Gamma_correction&oldid=751281772#Windows.2C_Mac.2C_sRGB_and_TV.2Fvideo_standard_gammas
    color=pow(color,vec3(1.,1.,1.)/2.2);
}

