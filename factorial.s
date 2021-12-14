;----------------;
; factorial of r0 ;
;----------------;
put r0 5; hardcoded input value

put r1 1; result

put r3 1; to compare with decreasing r0

@repeat;
    mul r1 r0;
    dec r0;

    cmp r0 r3;
    jne @repeat;

halt;
