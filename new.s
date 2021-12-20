; initialization
put r1 1;
put r2 5;
put r3 @loop;

@loop;
add r0 r1;
cmp r0 r2;
jne r3;
halt;