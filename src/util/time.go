package util

import (
	"fmt"
	"reflect"
	"time"
)

func TimeIt(name string, fn interface{}, args ...interface{}) (time.Duration, []reflect.Value, error) {
	// Convert the function to a reflect.Value
	fnVal := reflect.ValueOf(fn)
	if fnVal.Kind() != reflect.Func {
		return 0, nil, fmt.Errorf("expected a function, got %T", fn)
	}

	// Convert arguments to reflect.Values
	in := make([]reflect.Value, len(args))
	for i, arg := range args {
		in[i] = reflect.ValueOf(arg)
	}

	// Time the function execution
	start := time.Now()
	results := fnVal.Call(in)
	elapsed := time.Since(start)

	fmt.Printf("%s took %d", name, elapsed)

	// Return the execution time and the results
	return elapsed, results, nil
}
