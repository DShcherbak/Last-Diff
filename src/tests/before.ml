(* A comment that might take a few rows *)

let rec factorial n = if n = 0 then 1 else n * factorial (n-1) ;;

let rec is_even = function
        | 0 -> true
        | n -> is_odd (n-1)
and is_odd = function
        | 0 -> false
        | n -> is_even (n-1)
;;

let some_func x y = x + 4 ;;

let forth_power x = x * x * x * x;;