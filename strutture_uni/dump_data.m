clear
close all 
clc

E = [210e9; 105e9; 73e9;];
A = 2*pi.*[5e-2*5e-3; 3e-2*5e-3];

%nodes coordinates in the Global RF
Pn = [0.0, 0.0;
      1.5, 0.0;
      1.5, 1.0;
      3.0, 0.0;
      3.0, 2.0;
      4.5, 0.0;
      4.5, 3.0;
      6.0, 0.0;
      6.0, 2.0;
      7.5, 0.0;
      7.5, 1.0;
      9.0, 0.0;
    ];

%Connectivity Matrix
Cne = [ 1, 2;
        1, 3;
        2, 3;
        2, 4;
        3, 4; 
        3, 5;
        4, 5;
        4, 6;
        4, 7;
        5, 7;
        6, 7;
        6, 8;
        7, 8;
        7, 9;
        8, 9;
        8, 10;
        8, 11;
        9, 11;
        10, 11;
        10, 12;
        11, 12;
    ];


a1 = [A(1), E(1)];
a2 = [A(1), E(2)];
a3 = [A(1), E(3)];
b1 = [A(2), E(1)];
b2 = [A(2), E(2)];
b3 = [A(2), E(3)];


%characteristics of various elements 
Cpe = [ b3;
        b1;
        a2;
        b3;
        b1;
        b1;
        a2;
        b3;
        b1;
        b1;
        a2;
        b3;
        b1;
        b1;
        a2;
        b3;
        b1;
        b1;
        a2;
        b3;
        b1;
    ];

%blocked degree(s) of freedom 
ib = [1; 2; 23; 24;];

%applied loads
     %degree of freedom   load   
Fn = [        14,         -3e6;
              18,         -2e6;
              22,         -1e6;
    ];      














