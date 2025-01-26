import Handlebars from 'handlebars';
import { Template } from '../types';

// Register custom helper functions
Handlebars.registerHelper('formatDate', function(date: Date) {
  return new Date(date).toLocaleDateString();
});

Handlebars.registerHelper('formatAmount', function(amount: number) {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD'
  }).format(amount);
});

export async function renderTemplate(
  template: Template,
  data: any
): Promise<string> {
  try {
    // Compile template
    const compiled = Handlebars.compile(template.content);
    
    // Render template
    return compiled({
      ...data,
      timestamp: new Date(),
      year: new Date().getFullYear()
    });
  } catch (error) {
    console.error('Failed to render template:', error);
    throw error;
  }
}

export async function loadTemplate(type: string): Promise<Template> {
  try {
    // Load template from database or filesystem
    return {
      id: type,
      content: '',
      subject: '',
      variables: []
    };
  } catch (error) {
    console.error('Failed to load template:', error);
    throw error;
  }
}

export async function validateTemplate(
  template: Template,
  data: any
): Promise<boolean> {
  try {
    // Validate if all required variables exist
    const missingVars = template.variables.filter(
      variable => !(variable in data)
    );

    if (missingVars.length > 0) {
      throw new Error(`Missing required variables: ${missingVars.join(', ')}`);
    }

    // Try to render template
    await renderTemplate(template, data);
    return true;
  } catch (error) {
    console.error('Template validation failed:', error);
    return false;
  }
} 