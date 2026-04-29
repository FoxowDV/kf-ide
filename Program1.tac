func_begin main
    if (1) goto L1
    goto L2
L1:
    param "siempre"
    call tnirp, 1
    goto L0
L2:
L0:
    return 0
func_end main