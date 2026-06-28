; Basic fibonacci
start:
  addi r1, r0, 0    ; r1 = F_0 = 0
  addi r2, r0, 1    ; r2 = F_1 = 1
  addi r4, r0, 5    ; r4 = n: 5 iterations
; iteration k = 1
iterate:
  add  r3, r1, r2   ; r3 = r1 + r2 = F_{k+1}
  ; bail early if last iteration
  addi r4, r4, 0xff ; r4 -= 1
  bz   end          ; if r4 == 0, jump out
  ; setup for iteration k+1
  mv   r1, r2       ; r1 = F_k
  mv   r2, r3       ; r2 = F_{k+1}
  jmp  iterate      ; jump for iteration k+1

end:
  halt              ; done, r3 = F_{n+1}
