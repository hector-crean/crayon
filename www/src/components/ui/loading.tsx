"use client"

import { useState, useEffect } from "react"
import { motion, AnimatePresence } from "framer-motion"
import { Loader2 } from "lucide-react"

interface LoadingProps {
    isLoading: boolean
}
export const Loading = ({isLoading}: LoadingProps) => {

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-background z-50">
      <AnimatePresence>
        {isLoading && (
          <motion.div
            initial={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.5 }}
            className="fixed inset-0 flex items-center justify-center bg-background"
          >
            <Loader2 className="w-10 h-10 text-primary animate-spin" />
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  )
}