'use client'

import { useTypography } from '@/hooks/use-typography'
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Slider } from "@/components/ui/slider"

export function TypographyController() {
  const { 
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
  } = useTypography()

  return (
    <div className="space-y-6">
      <div className="space-y-2">
        <Label>Preview</Label>
        <div 
          className="p-4 border rounded-md"
          style={{
            fontSize: `${fontSize}px`,
            lineHeight: lineHeight,
            letterSpacing: `${letterSpacing}px`,
            fontWeight: fontWeight
          }}
        >
          The quick brown fox jumps over the lazy dog.
        </div>
      </div>

      <div className="space-y-2">
        <Label htmlFor="font-size">Font Size</Label>
        <div className="flex items-center space-x-2">
          <Button onClick={decreaseFontSize} variant="outline" size="icon">-</Button>
          <Input
            id="font-size"
            type="number"
            value={fontSize}
            onChange={(e) => {
              const value = parseInt(e.target.value)
              if (value >= 12 && value <= 24) {
                setFontSize(value)
              }
            }}
            className="w-20"
          />
          <Button onClick={increaseFontSize} variant="outline" size="icon">+</Button>
        </div>
      </div>

      <div className="space-y-2">
        <Label htmlFor="line-height">Line Height</Label>
        <Slider
          id="line-height"
          min={1}
          max={2}
          step={0.1}
          value={[lineHeight]}
          onValueChange={(value) => setLineHeight(value[0])}
        />
        <div className="text-sm text-muted-foreground">{lineHeight.toFixed(1)}</div>
      </div>

      <div className="space-y-2">
        <Label htmlFor="letter-spacing">Letter Spacing</Label>
        <Slider
          id="letter-spacing"
          min={-2}
          max={10}
          step={0.5}
          value={[letterSpacing]}
          onValueChange={(value) => setLetterSpacing(value[0])}
        />
        <div className="text-sm text-muted-foreground">{letterSpacing.toFixed(1)}px</div>
      </div>

      <div className="space-y-2">
        <Label htmlFor="font-weight">Font Weight</Label>
        <Slider
          id="font-weight"
          min={100}
          max={900}
          step={100}
          value={[fontWeight]}
          onValueChange={(value) => setFontWeight(value[0])}
        />
        <div className="text-sm text-muted-foreground">{fontWeight}</div>
      </div>
    </div>
  )
}

