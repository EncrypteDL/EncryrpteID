package tracing

import (
	"github.com/ethereum/go-ethereum/eth/tracers/logger"
	"go.opentelemetry.io/otel/sdk/trace"
)

type endpointExcluder struct {
	log         *logger.Config
	endPoints   map[string]struct{}
	probability float64
}

func newEndpointExcluder(log *logger.Config, endpoints map[string]struct{}, probability float64) endpointExcluder {
	return endpointExcluder{
		log:         log,
		endPoints:   endpoints,
		probability: probability,
	}
}

// ShouldSample implements the sampler interface. It prevents the specified
// endpoints from being added to the trace.
func (ee endpointExcluder) ShouldSample(parameters trace.SamplingParameters) trace.SamplingResult {
	for i := range parameters.Attributes {
		if parameters.Attributes[i].Key == "http.target" {
			if _, exists := ee.endPoints[parameters.Attributes[i].Value.AsString()]; exists {
				return trace.SamplingResult{Decision: trace.Drop}
			}
		}
	}

	return trace.TraceIDRatioBased(ee.probability).ShouldSample(parameters)
}

// Description implements the sampler interface.
func (endpointExcluder) Description() string {
	return "customSampler"
}
