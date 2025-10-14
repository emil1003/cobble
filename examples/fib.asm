; Basic fibonacci
start:
  addi r1, r0, 1    ; a = 1
  addi r2, r0, 1    ; b = 1
  addi r4, r0, 5    ; d = 5, n iterations
loop:
  add  r3, r1, r2   ; c = a + b
  mv   r1, r2       ; a = b
  mv   r2, r3       ; b = c

  addi r4, r4, 0xff ; d -= 1
  bnz  loop         ; jump to loop if not done
  halt              ; done, result in r3
