"use client";

import * as React from "react";
import { useState, useRef, useEffect } from "react";
import { motion, AnimatePresence, Variants } from "framer-motion";
import { Button } from "./button";
import {
  ArrowRight,
  ChevronDown,
  ChevronUp,
  ChevronUpIcon,
} from "lucide-react";
import { TextAreaFormSuccessEvent } from "../blocks/text-area-form";

export const wrap = (min: number, max: number, v: number) => {
  const rangeSize = max - min;
  return ((((v - min) % rangeSize) + rangeSize) % rangeSize) + min;
};

// Variants for slide transitions
const variants: Variants = {
  enter: (direction: number) => ({
    y: direction > 0 ? 1000 : -1000,
    opacity: 0,
    zIndex: 0,
  }),
  center: {
    zIndex: 1,
    y: 0,
    opacity: 1,
  },
  exit: (direction: number) => ({
    zIndex: 0,
    y: direction < 0 ? 1000 : -1000,
    opacity: 0,
  }),
};

/**
 * The absolute offset * velocity swiping heuristic
 */
const swipeConfidenceThreshold = 10000;
const swipePower = (offset: number, velocity: number) => {
  return Math.abs(offset) * velocity;
};

type SlideDeckProps<T> = {
  slides: T[];
  title: (slide: T) => React.ReactNode;
  render: (slide: T) => React.ReactNode;
};

export function SlideDeck<T>({
  slides,
  title,
  render,
}: SlideDeckProps<T>): React.ReactNode {
  const [[slide, direction], setPage] = useState<[number, number]>([0, 0]);

  const slideIdx = wrap(0, slides.length, slide);

  const containerRef = useRef<HTMLDivElement>(null);

  // Prevent multiple scroll triggers in a short time.
  const [lastScrollTime, setLastScrollTime] = useState<number>(0);
  const scrollCooldown = 800; // milliseconds

  /** Move to next or previous slide */
  const paginate = (newDirection: number) => {
    setPage(([prevPage]) => {
      const newPage = prevPage + newDirection;
      if (newPage < 0 || newPage >= slides.length) {
        // Prevent looping
        return [prevPage, newDirection];
      }
      return [newPage, newDirection];
    });
  };

  /** Handle swiping left/right in the drag end callback */
  const handleDragEnd = (
    e: PointerEvent | MouseEvent | TouchEvent,
    {
      offset,
      velocity,
    }: { offset: { x: number; y: number }; velocity: { x: number; y: number } }
  ) => {
    const swipe = swipePower(offset.y, velocity.y);

    if (swipe < -swipeConfidenceThreshold) {
      paginate(1);
    } else if (swipe > swipeConfidenceThreshold) {
      paginate(-1);
    }
  };

  /** Handle wheel scrolling, scrolling down => next, up => prev (with threshold) */
  const handleWheel = (e: React.WheelEvent<HTMLDivElement>) => {
    const now = Date.now();
    if (now - lastScrollTime < scrollCooldown) {
      // Too soon, ignore
      return;
    }

    const scrollThreshold = 50;
    if (e.deltaY > scrollThreshold) {
      paginate(1);
      setLastScrollTime(now);
    } else if (e.deltaY < -scrollThreshold) {
      paginate(-1);
      setLastScrollTime(now);
    }
  };

  /** Handle keyboard input (left/right or up/down) */
  const handleKeyDown = (e: React.KeyboardEvent<HTMLDivElement>) => {
    switch (e.key) {
      case "ArrowRight":
      case "ArrowDown":
      case "Enter":
        paginate(1);
        break;
      case "ArrowLeft":
      case "ArrowUp":
        paginate(-1);
        break;
      default:
        break;
    }
  };

  useEffect(() => {
    // Optionally auto-focus the container so key events work without extra clicks
    containerRef.current?.focus();
  }, []);

  useEffect(() => {
    const handleSuccessEvent = (
      event: CustomEvent<TextAreaFormSuccessEvent>
    ) => {
      console.log("Success event received:", event.detail.data);
      // Handle the event, e.g., update state or perform some action
      paginate(1);
    };

    // Add event listener for the custom event
    window.addEventListener(
      "text-area-form.success",
      handleSuccessEvent as EventListener
    );

    // Clean up the event listener on component unmount
    return () => {
      window.removeEventListener(
        "text-area-form.success",
        handleSuccessEvent as EventListener
      );
    };
  }, []);

  return (
    <div
      ref={containerRef}
      tabIndex={0}
      onKeyDown={handleKeyDown}
      onWheel={handleWheel}
      className="relative flex items-center justify-center w-full h-full overflow-hidden bg-background text-foreground focus:outline-hidden"
    >
      {/* Progress Bar */}
      <div className="absolute top-0 left-0 w-full h-1 bg-gray-200">
        <motion.div
          className="h-full bg-accent"
          animate={{ width: `${((slideIdx + 1) / slides.length) * 100}%` }}
          transition={{ duration: 0.5 }}
        />
      </div>
      <AnimatePresence initial={false} custom={direction}>
        <motion.div
          key={slideIdx}
          custom={direction}
          variants={variants}
          initial="enter"
          animate="center"
          exit="exit"
          transition={{
            x: { type: "spring", stiffness: 300, damping: 30 },
            opacity: { duration: 0.2 },
          }}
          drag="x"
          dragConstraints={{ left: 0, right: 0 }}
          dragElastic={1}
          onDragEnd={handleDragEnd}
          className="absolute w-auto max-w-full max-h-full grid grid-cols-[min_content_1fr] gap-4"
        >
          <div className="flex flex-row items-center justify-start gap-1">
            {/* {slideIdx} <span className="text-muted-foreground"><ArrowRight width={16} height={16}/></span> */}
            {/* {title(slides[slideIdx])} */}
          </div>
          <div className="flex flex-col items-center justify-start w-full">
            {render(slides[slideIdx])}
          </div>
        </motion.div>
      </AnimatePresence>

      {/* Next/Prev Buttons */}
      <div className="absolute bottom-4 right-4 flex gap-2 flex-row">
        <Button variant={"ghost"} size={"icon"} onClick={() => paginate(-1)}>
          <ChevronUp />
        </Button>
        <Button variant={"ghost"} size={"icon"} onClick={() => paginate(1)}>
          <ChevronDown />
        </Button>
      </div>
    </div>
  );
}
