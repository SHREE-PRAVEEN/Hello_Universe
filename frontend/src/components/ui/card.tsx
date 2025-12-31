'use client';

import * as React from 'react';
import { cn } from '@/lib/utils';

// ============================================
// CARD COMPONENT
// ============================================

export interface CardProps extends React.HTMLAttributes<HTMLDivElement> {
  variant?: 'default' | 'elevated' | 'outlined' | 'glass';
  hover?: boolean;
}

const Card = React.forwardRef<HTMLDivElement, CardProps>(
  ({ className, variant = 'default', hover = false, ...props }, ref) => {
    const variants = {
      default: 'bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800',
      elevated: 'bg-white dark:bg-zinc-900 shadow-xl shadow-zinc-900/5 dark:shadow-black/20',
      outlined: 'bg-transparent border-2 border-zinc-200 dark:border-zinc-700',
      glass:
        'bg-white/10 dark:bg-zinc-900/50 backdrop-blur-xl border border-white/20 dark:border-zinc-700/50',
    };

    return (
      <div
        ref={ref}
        className={cn(
          'rounded-2xl transition-all duration-300',
          variants[variant],
          hover && 'hover:scale-[1.02] hover:shadow-2xl cursor-pointer',
          className
        )}
        {...props}
      />
    );
  }
);

Card.displayName = 'Card';

// ============================================
// CARD HEADER
// ============================================

const CardHeader = React.forwardRef<HTMLDivElement, React.HTMLAttributes<HTMLDivElement>>(
  ({ className, ...props }, ref) => (
    <div
      ref={ref}
      className={cn('flex flex-col space-y-1.5 p-6', className)}
      {...props}
    />
  )
);

CardHeader.displayName = 'CardHeader';

// ============================================
// CARD TITLE
// ============================================

const CardTitle = React.forwardRef<HTMLHeadingElement, React.HTMLAttributes<HTMLHeadingElement>>(
  ({ className, ...props }, ref) => (
    <h3
      ref={ref}
      className={cn(
        'text-xl font-semibold leading-none tracking-tight text-zinc-900 dark:text-zinc-50',
        className
      )}
      {...props}
    />
  )
);

CardTitle.displayName = 'CardTitle';

// ============================================
// CARD DESCRIPTION
// ============================================

const CardDescription = React.forwardRef<
  HTMLParagraphElement,
  React.HTMLAttributes<HTMLParagraphElement>
>(({ className, ...props }, ref) => (
  <p
    ref={ref}
    className={cn('text-sm text-zinc-500 dark:text-zinc-400', className)}
    {...props}
  />
));

CardDescription.displayName = 'CardDescription';

// ============================================
// CARD CONTENT
// ============================================

const CardContent = React.forwardRef<HTMLDivElement, React.HTMLAttributes<HTMLDivElement>>(
  ({ className, ...props }, ref) => (
    <div ref={ref} className={cn('p-6 pt-0', className)} {...props} />
  )
);

CardContent.displayName = 'CardContent';

// ============================================
// CARD FOOTER
// ============================================

const CardFooter = React.forwardRef<HTMLDivElement, React.HTMLAttributes<HTMLDivElement>>(
  ({ className, ...props }, ref) => (
    <div
      ref={ref}
      className={cn('flex items-center p-6 pt-0', className)}
      {...props}
    />
  )
);

CardFooter.displayName = 'CardFooter';

// ============================================
// FEATURE CARD (Pre-styled variant)
// ============================================

export interface FeatureCardProps {
  icon: React.ReactNode;
  title: string;
  description: string;
  className?: string;
}

const FeatureCard = ({ icon, title, description, className }: FeatureCardProps) => (
  <Card variant="glass" hover className={cn('group', className)}>
    <CardHeader>
      <div className="mb-4 inline-flex h-12 w-12 items-center justify-center rounded-xl bg-gradient-to-br from-cyan-500 to-blue-600 text-white transition-transform group-hover:scale-110">
        {icon}
      </div>
      <CardTitle>{title}</CardTitle>
    </CardHeader>
    <CardContent>
      <CardDescription className="text-base">{description}</CardDescription>
    </CardContent>
  </Card>
);

// ============================================
// STATS CARD (Pre-styled variant)
// ============================================

export interface StatsCardProps {
  label: string;
  value: string | number;
  change?: {
    value: number;
    trend: 'up' | 'down' | 'neutral';
  };
  icon?: React.ReactNode;
  className?: string;
}

const StatsCard = ({ label, value, change, icon, className }: StatsCardProps) => (
  <Card variant="elevated" className={cn('', className)}>
    <CardContent className="p-6">
      <div className="flex items-center justify-between">
        <div>
          <p className="text-sm font-medium text-zinc-500 dark:text-zinc-400">{label}</p>
          <p className="mt-1 text-3xl font-bold text-zinc-900 dark:text-zinc-50">{value}</p>
          {change && (
            <p
              className={cn(
                'mt-1 text-sm font-medium',
                change.trend === 'up' && 'text-emerald-500',
                change.trend === 'down' && 'text-red-500',
                change.trend === 'neutral' && 'text-zinc-500'
              )}
            >
              {change.trend === 'up' && '↑'}
              {change.trend === 'down' && '↓'}
              {change.value}%
            </p>
          )}
        </div>
        {icon && (
          <div className="flex h-12 w-12 items-center justify-center rounded-xl bg-zinc-100 text-zinc-600 dark:bg-zinc-800 dark:text-zinc-400">
            {icon}
          </div>
        )}
      </div>
    </CardContent>
  </Card>
);

export {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
  CardFooter,
  FeatureCard,
  StatsCard,
};
