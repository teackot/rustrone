; initialization
put r1 1;
put r2 @target_value;
ldr r2 r2;
put r3 @loop;

@loop;
add r0 r1;
cmp r0 r2;
jne r3;
halt;

; some raw data
@target_value;
#50;