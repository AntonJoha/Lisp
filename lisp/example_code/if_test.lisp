( def fun ( a )
	( if 
		( == a 1 )
		1
		( + a ( fun ( - a 1 ) ) )
	)
)

(fun 200 ) 
( fun 5 ) 
