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

	fmt.Printf("\n%s took %d ms", name, elapsed)
	// Print the elapsed time in hours, minutes, and seconds
	hours := int(elapsed.Hours())
	minutes := int(elapsed.Minutes()) % 60
	seconds := int(elapsed.Seconds()) % 60
	fmt.Printf("\nAKA:\n%s took %d hours %d minutes %d seconds\n", name, hours, minutes, seconds)
	// Return the execution time and the results
	return elapsed, results, nil
}
