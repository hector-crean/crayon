'use client'

import { useState, useEffect, useRef, useCallback } from 'react'

const MIN_FONT_SIZE = 1
const MAX_FONT_SIZE = 24
const DEFAULT_FONT_SIZE = 16

const BREAKPOINTS = {
  sm: 640,
  md: 768,
  lg: 1024,
  xl: 1280,
  '2xl': 1536,
} as const

const RESIZE_DEBOUNCE_DELAY = 250

function useDebounce<T extends (...args: any[]) => void>(
  callback: T,
  delay: number
): T {
  const timeoutRef = useRef<number | undefined>(undefined)

  useEffect(() => {
    return () => {
      if (timeoutRef.current) {
        window.clearTimeout(timeoutRef.current)
      }
    }
  }, [])

  return useCallback(
    (...args: Parameters<T>) => {
      if (timeoutRef.current) {
        window.clearTimeout(timeoutRef.current)
      }
      timeoutRef.current = window.setTimeout(() => {
        callback(...args)
      }, delay)
    },
    [callback, delay]
  ) as T
}

const getDefaultFontSize = () => {
  if (typeof window === 'undefined') return DEFAULT_FONT_SIZE
  
  const width = window.innerWidth
  if (width < BREAKPOINTS.sm) return 12      // Mobile small
  if (width < BREAKPOINTS.md) return 14      // Mobile large/Tablet small
  if (width < BREAKPOINTS.lg) return 16      // Tablet
  if (width < BREAKPOINTS.xl) return 16      // Desktop
  if (width < BREAKPOINTS['2xl']) return 16  // Large Desktop
  return 16                                // Extra Large Desktop
}

export function useTypography() {
  const [fontSize, setFontSize] = useState(getDefaultFontSize())
  const [lineHeight, setLineHeight] = useState(1.5)
  const [letterSpacing, setLetterSpacing] = useState(0)
  const [fontWeight, setFontWeight] = useState(400)

  useEffect(() => {
    document.documentElement.style.setProperty('--base-font-size', `${fontSize}px`)
    document.documentElement.style.setProperty('--base-line-height', lineHeight.toString())
    document.documentElement.style.setProperty('--base-letter-spacing', `${letterSpacing}px`)
    document.documentElement.style.setProperty('--base-font-weight', fontWeight.toString())

  }, [fontSize, lineHeight, letterSpacing, fontWeight])

  const handleResize = useDebounce(() => {
    const size = getDefaultFontSize()
    console.log('size', size)
    setFontSize(size)
  }, RESIZE_DEBOUNCE_DELAY)

  useEffect(() => {
    window.addEventListener('resize', handleResize)
    return () => window.removeEventListener('resize', handleResize)
  }, [handleResize])

  const increaseFontSize = () => setFontSize(prevSize => Math.min(prevSize + 2, MAX_FONT_SIZE))
  const decreaseFontSize = () => setFontSize(prevSize => Math.max(prevSize - 2, MIN_FONT_SIZE))

  return { 
    fontSize, 
    increaseFontSize, 
    decreaseFontSize,
    setFontSize,
    lineHeight,
    setLineHeight,
    letterSpacing,
    setLetterSpacing,
    fontWeight,
    setFontWeight
  }
}


export { getDefaultFontSize}