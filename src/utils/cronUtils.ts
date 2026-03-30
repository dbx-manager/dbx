/**
 * Utility functions for parsing and formatting cron expressions
 */

/**
 * Parses a cron expression and extracts hour and minute components
 * @param cron - Cron expression string (e.g., "1 10 * * *")
 * @returns Object with hour and minute, or null if invalid
 */
export function parseCronTime(cron: string | null): { hour: number; minute: number } | null {
  if (!cron || typeof cron !== 'string') {
    return null;
  }

  // Split cron expression by spaces
  const parts = cron.trim().split(/\s+/);
  
  // Standard cron format: minute hour day month dayOfWeek
  // We need at least minute and hour (first 2 parts)
  if (parts.length < 2) {
    return null;
  }

  try {
    const minute = parseInt(parts[0], 10);
    const hour = parseInt(parts[1], 10);

    // Validate ranges
    if (isNaN(minute) || isNaN(hour)) {
      return null;
    }

    if (minute < 0 || minute > 59) {
      return null;
    }

    if (hour < 0 || hour > 23) {
      return null;
    }

    return { hour, minute };
  } catch (error) {
    console.warn('Failed to parse cron expression:', cron, error);
    return null;
  }
}

/**
 * Converts 24-hour time to 12-hour format with AM/PM
 * @param hour - Hour in 24-hour format (0-23)
 * @param minute - Minute (0-59)
 * @returns Formatted time string (e.g., "10:01 AM")
 */
export function formatTimeTo12Hour(hour: number, minute: number): string {
  // Validate inputs
  if (hour < 0 || hour > 23 || minute < 0 || minute > 59) {
    return 'Invalid time';
  }

  // Convert to 12-hour format
  const period = hour >= 12 ? 'PM' : 'AM';
  const displayHour = hour === 0 ? 12 : (hour > 12 ? hour - 12 : hour);
  const displayMinute = minute.toString().padStart(2, '0');

  return `${displayHour}:${displayMinute} ${period}`;
}

/**
 * Formats a cron expression to human-readable time
 * @param cron - Cron expression string
 * @returns Formatted time string or error message
 */
export function formatCronToTime(cron: string | null): string {
  if (!cron) {
    return 'No schedule set';
  }

  const time = parseCronTime(cron);
  
  if (!time) {
    return 'Invalid schedule';
  }

  return formatTimeTo12Hour(time.hour, time.minute);
}

/**
 * Parses a cron expression and extracts date components
 * @param cron - Cron expression string (e.g., "1 10 * * *")
 * @returns Object with day, month, dayOfWeek, or null if invalid
 */
export function parseCronDate(cron: string | null): { day: string; month: string; dayOfWeek: string } | null {
  if (!cron || typeof cron !== 'string') {
    return null;
  }

  // Split cron expression by spaces
  const parts = cron.trim().split(/\s+/);
  
  // Standard cron format: minute hour day month dayOfWeek
  // We need at least minute, hour, and day (first 3 parts)
  if (parts.length < 3) {
    return null;
  }

  try {
    const day = parts[2]; // Day of month (1-31) or *
    const month = parts[3]; // Month (1-12) or *
    const dayOfWeek = parts[4]; // Day of week (0-6) or *

    return { day, month, dayOfWeek };
  } catch (error) {
    console.warn('Failed to parse cron date components:', cron, error);
    return null;
  }
}

/**
 * Converts day of month to ordinal format (1st, 2nd, 3rd, 4th, etc.)
 * @param day - Day number as string
 * @returns Ordinal day string
 */
function getOrdinalDay(day: string): string {
  const num = parseInt(day, 10);
  if (isNaN(num) || num < 1 || num > 31) {
    return day;
  }

  const j = num % 10;
  const k = num % 100;
  
  if (j === 1 && k !== 11) {
    return num + "st";
  }
  if (j === 2 && k !== 12) {
    return num + "nd";
  }
  if (j === 3 && k !== 13) {
    return num + "rd";
  }
  
  return num + "th";
}

/**
 * Converts day of week number to day name
 * @param dayOfWeek - Day of week as string (0-6)
 * @returns Day name string
 */
function getDayOfWeekName(dayOfWeek: string): string {
  const days = [
    'Sunday', 'Monday', 'Tuesday', 'Wednesday', 
    'Thursday', 'Friday', 'Saturday'
  ];
  
  const num = parseInt(dayOfWeek, 10);
  if (isNaN(num) || num < 0 || num > 6) {
    return dayOfWeek;
  }
  
  return days[num];
}

/**
 * Formats a cron expression to human-readable date
 * @param cron - Cron expression string
 * @returns Formatted date string or error message
 */
export function formatCronToDate(cron: string | null): string {
  if (!cron) {
    return 'No schedule set';
  }

  const date = parseCronDate(cron);
  
  if (!date) {
    return 'Invalid schedule';
  }

  // Check if it's a daily schedule (all date fields are *)
  if (date.day === '*' && date.month === '*' && date.dayOfWeek === '*') {
    return 'Daily';
  }

  // Check if it's a weekly schedule (day and month are *, dayOfWeek is specific)
  if (date.day === '*' && date.month === '*' && date.dayOfWeek !== '*') {
    const dayName = getDayOfWeekName(date.dayOfWeek);
    return `Weekly on ${dayName}`;
  }

  // Check if it's a monthly schedule (day is specific, month and dayOfWeek are *)
  if (date.day !== '*' && date.month === '*' && date.dayOfWeek === '*') {
    const ordinalDay = getOrdinalDay(date.day);
    return `Monthly on the ${ordinalDay}`;
  }

  // For other complex schedules, show a generic format
  return 'Custom schedule';
}
