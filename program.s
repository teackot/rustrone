put r0 5;
put r2 40;
str r0 r2;

put r0 2;
put r3 41;
str r0 r3;

ldr r0 r2;
ldr r1 r3;

sub r0 r1;

put r3 3;
cmp r0 r3;
je 16;

halt;
