import { useEffect, useState, ReactNode } from 'react';
import { responsiveConfig } from '../../config/responsive.config';

interface ResponsiveLayoutProps {
  children: ReactNode;
  className?: string;
}

export const ResponsiveLayout = ({ children, className = '' }: ResponsiveLayoutProps) => {
  const [breakpoint, setBreakpoint] = useState(getInitialBreakpoint());

  useEffect(() => {
    const handleResize = () => {
      const width = window.innerWidth;
      const newBreakpoint = Object.entries(responsiveConfig.grid.breakpoints)
        .find(([_, { min, max }]) => width >= min && width <= max)?.[0] || 'xs';
      
      if (newBreakpoint !== breakpoint) {
        setBreakpoint(newBreakpoint);
      }
    };

    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, [breakpoint]);

  function getInitialBreakpoint() {
    const width = window.innerWidth;
    return Object.entries(responsiveConfig.grid.breakpoints)
      .find(([_, { min, max }]) => width >= min && width <= max)?.[0] || 'xs';
  }

  return (
    <div 
      className={`responsive-layout ${breakpoint} ${className}`}
      style={{
        padding: responsiveConfig.container.padding[breakpoint as keyof typeof responsiveConfig.container.padding],
        maxWidth: responsiveConfig.container.maxWidth[breakpoint as keyof typeof responsiveConfig.container.maxWidth]
      }}
    >
      {children}
    </div>
  );
}; 