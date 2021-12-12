;----------------;
; factorial of 5 ;
;----------------;
put r0 5;
put r1 1;
put r2 1; to decrement
put r3 0; to compare with decreasing r0

@repeat;
mul r1 r0;
sub r0 r2;

cmp r0 r3;
jne @repeat;

halt;
