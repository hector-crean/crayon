'use client'

import React, { useState, useCallback, useRef } from 'react'
import { X, Upload, AlertCircle } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Alert, AlertDescription } from '@/components/ui/alert'
import { useForm, Controller } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { z } from 'zod'
import { ScrollArea } from './scroll-area'

// Define a Zod schema for file validation
const fileSchema = z.object({
  files: z.array(
    z.object({
      name: z.string(),
      size: z.number().max(5 * 1024 * 1024, "File size should not exceed 5MB"),
      type: z.string().refine((type) => ['image/*', 'application/pdf'].some((acceptedType) => type.match(acceptedType)), "Invalid file type")
    })
  )
})

export interface FileWithPreview extends File {
  preview?: string;
}

export interface DropzoneProps {
  onFilesAdded: (files: FileWithPreview[]) => void;
}

export const Dropzone: React.FC<DropzoneProps> = ({ onFilesAdded }) => {
  const [files, setFiles] = useState<FileWithPreview[]>([])
  const [text, setText] = useState<string>('')
  const [isDragging, setIsDragging] = useState(false)
  const fileInputRef = useRef<HTMLInputElement>(null)

  const { control, handleSubmit, setError, clearErrors } = useForm({
    resolver: zodResolver(fileSchema),
    defaultValues: { files: [] }
  })

  const handleFiles = useCallback(
    (newFiles: File[]) => {
      const filesWithPreviews = newFiles.map((file) => 
        Object.assign(file, { preview: URL.createObjectURL(file) })
      )

      setFiles((prevFiles) => [...prevFiles, ...filesWithPreviews])
      onFilesAdded(filesWithPreviews)
      clearErrors('files')
    },
    [onFilesAdded, clearErrors]
  )

  const handleDrop = useCallback(
    (event: React.DragEvent<HTMLDivElement>) => {
      event.preventDefault()
      setIsDragging(false)
      const droppedFiles = Array.from(event.dataTransfer.files)
      handleFiles(droppedFiles)
    },
    [handleFiles]
  )

  const handlePaste = useCallback(
    (event: React.ClipboardEvent<HTMLDivElement>) => {
      const pastedFiles = Array.from(event.clipboardData.files)
      if (pastedFiles.length > 0) {
        handleFiles(pastedFiles)
      } else {
        const pastedText = event.clipboardData.getData('Text')
        setText((prevText) => prevText + pastedText)
      }
    },
    [handleFiles]
  )

  const handleDragOver = useCallback((event: React.DragEvent<HTMLDivElement>) => {
    event.preventDefault()
    setIsDragging(true)
  }, [])

  const handleDragLeave = useCallback(() => {
    setIsDragging(false)
  }, [])

  const handleClick = () => {
    fileInputRef.current?.click()
  }

  const handleFileInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const selectedFiles = Array.from(event.target.files || [])
    handleFiles(selectedFiles)
  }

  const removeFile = (fileToRemove: FileWithPreview) => (e: React.PointerEvent<HTMLButtonElement>) => {
    e.stopPropagation()
    setFiles((prevFiles) => prevFiles.filter((file) => file !== fileToRemove))
    URL.revokeObjectURL(fileToRemove.preview || '')
  }

  const formatFileSize = (bytes: number) => {
    if (bytes < 1024) return bytes + ' bytes'
    else if (bytes < 1048576) return (bytes / 1024).toFixed(1) + ' KB'
    else return (bytes / 1048576).toFixed(1) + ' MB'
  }

  return (
    <form onSubmit={handleSubmit(() => {})} className="w-full h-full flex flex-col items-center justify-center p-4">
      <div
        onDrop={handleDrop}
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onPaste={handlePaste}
        onClick={handleClick}
        className={`group flex flex-col items-center justify-center p-6 w-full h-full rounded-lg text-center cursor-pointer transition-colors ${
          isDragging ? 'bg-secondary text-secondary-foreground' : 'bg-destructive text-destructive-foreground hover:bg-secondary hover:text-secondary-foreground'
        }`}
      >
        <input
          ref={fileInputRef}
          type="file"
          multiple
          onChange={handleFileInputChange}
          accept="image/*,application/pdf"
          className="hidden"
        />
        <Upload className="mx-auto h-12 w-12 transition-transform duration-300 " />
        <p className="mt-2 text-sm">
          Drag and drop files here, click to select files, or paste
        </p>
        <Controller
          name="files"
          control={control}
          render={({ fieldState }) => (
            fieldState.error ? (
              <Alert variant="destructive" className="mt-4">
                <AlertCircle className="h-4 w-4" />
                <AlertDescription>{fieldState.error.message}</AlertDescription>
              </Alert>
            ) : null
          )}
        />
        {files.length > 0 && (
          <div className="mt-4">
            <h4 className="text-sm font-medium">Files:</h4>
            <ScrollArea className="h-72 p-4 w-full rounded-md shadow-md">

            <ul className="mt-2">
              {files.map((file, index) => (
                <li key={index} className="py-2 flex items-center justify-between">
                  <div className="flex items-center">
                    {file.type.startsWith('image/') && (
                      <img src={file.preview} alt={file.name} className="h-8 w-8 object-cover mr-2 rounded" />
                    )}
                    <span className="text-sm">{file.name}</span>
                  </div>
                  <div className="flex items-center">
                    <span className="text-xs mr-2">{formatFileSize(file.size)}</span>
                    <Button variant='outline' size="icon" onClick={removeFile(file)}>
                      <X className="h-4 w-4" />
                    </Button>
                  </div>
                </li>
              ))}
            </ul>
            </ScrollArea>
          </div>
        )}
        {text && (
          <div className="mt-4">
            <h4 className="text-sm font-medium ">Text:</h4>
            <p className="mt-2 text-sm">{text}</p>
          </div>
        )}
      </div>
    </form>
  )
}

