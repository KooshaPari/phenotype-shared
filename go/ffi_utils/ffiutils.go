package ffiutils

// This package provides a thin wrapper for safe mutex use across FFI boundaries.
// For now it exposes a Go wrapper around sync.Mutex behaviors. We may expand to
// use specialized concurrency primitives or cgo for interoperability.

import "sync"

type FfiMutex struct {
	mu sync.Mutex
}

func New() *FfiMutex { return &FfiMutex{} }

func (m *FfiMutex) Lock() { m.mu.Lock() }

func (m *FfiMutex) Unlock() { m.mu.Unlock() }
