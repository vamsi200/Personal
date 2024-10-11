#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#define MAX_TEMPERATURE 80.00
#define FREQUENCY_DECREMENT 500000
#define WAIT_TIME_AFTER_THROTTLE 15
#define WAIT_TIME_AFTER_TEMP_DROP 5
#define MAX_ATTEMPTS 5

float cpu_temp;

float get_cpu_temp(void) {
  FILE *open_file = fopen("/sys/class/thermal/thermal_zone0/temp", "r");
  if (open_file == NULL) {
    perror("Error: Couldn't Read CPU Temperature");
    return -1;
  } else {
    fscanf(open_file, "%f", &cpu_temp);
    fclose(open_file);
    return cpu_temp / 1000.00;
  }
}

int get_current_cpu_frequency(int cpu) {
  char current_cpu_frequency[256];
  snprintf(current_cpu_frequency, sizeof(current_cpu_frequency),
           "/sys/devices/system/cpu/cpu%d/cpufreq/scaling_max_freq", cpu);

  FILE *fp_max = fopen(current_cpu_frequency, "r");
  if (fp_max == NULL) {
    perror("Error: Couldn't Read CPU Frequency");
    return -1;
  }

  int max_scaling_freq;
  if (fscanf(fp_max, "%d", &max_scaling_freq) != 1) {
    perror("Error: reading max scaling frequency");
    fclose(fp_max);
    return -1;
  }

  fclose(fp_max);
  return max_scaling_freq;
}

int change_cpu_frequency(int cpu, int frequency) {
  char current_cpu_frequency[256];
  snprintf(current_cpu_frequency, sizeof(current_cpu_frequency),
           "/sys/devices/system/cpu/cpu%d/cpufreq/scaling_max_freq", cpu);

  FILE *fp_set = fopen(current_cpu_frequency, "w");
  if (fp_set == NULL) {
    perror("Error: Couldn't Set Throttled Frequency");
    return -1;
  }

  fprintf(fp_set, "%d", frequency);
  fclose(fp_set);
  return 0;
}

int get_original_cpu_frequency(int cpu) {
  char get_cpumax_fq_info[256];
  snprintf(get_cpumax_fq_info, sizeof(get_cpumax_fq_info),
           "/sys/devices/system/cpu/cpu%d/cpufreq/cpuinfo_max_freq", cpu);
  FILE *fp = fopen(get_cpumax_fq_info, "r");

  if (fp == NULL) {
    perror("Error: Couldn't read `CPUMAX_FQ_INFO`");
    return -1;
  }
  int original_cpu_freq;
  if (fscanf(fp, "%d", &original_cpu_freq) != 1) {
    perror("Error: Couldn't read `Original_Cpu_Freq`");
    fclose(fp);
    return -1;
  }

  fclose(fp);
  return original_cpu_freq;
}

int set_to_original_frequency(int cpu, int frequency) {
  char scaling_max_fq_info[256];
  snprintf(scaling_max_fq_info, sizeof(scaling_max_fq_info),
           "/sys/devices/system/cpu/cpu%d/cpufreq/scaling_max_freq", cpu);
  FILE *fp = fopen(scaling_max_fq_info, "w");
  if (fp == NULL) {
    perror("Error: Couldn't write `scaling_max_freq`");
    return -1;
  }
  fprintf(fp, "%d", frequency);
  fclose(fp);
  return 0;
}

int throttle() {
  int max_cpu = sysconf(_SC_NPROCESSORS_ONLN);
  float cpu_temp = get_cpu_temp();
  int attempt = 0;

  while (cpu_temp > MAX_TEMPERATURE && attempt < MAX_ATTEMPTS) {
    printf("[*] Throttle attempt %d/%d\n", attempt + 1, MAX_ATTEMPTS);

    for (int cpu = 0; cpu < max_cpu; cpu++) {
      int max_freq = get_current_cpu_frequency(cpu);
      if (max_freq < 0) {
        continue;
      }

      int throttle_freq = max_freq - FREQUENCY_DECREMENT;
      if (throttle_freq < 0) {
        throttle_freq = 0;
      }

      printf("CPU%d: Throttling frequency from %d Hz to %d Hz\n", cpu, max_freq,
             throttle_freq);
      if (change_cpu_frequency(cpu, throttle_freq) < 0) {
        continue;
      }
    }

    printf("[*] Waiting for %d seconds, Current temperature:%f...\n",
           WAIT_TIME_AFTER_THROTTLE, cpu_temp);
    sleep(WAIT_TIME_AFTER_THROTTLE);

    cpu_temp = get_cpu_temp();
    if (cpu_temp < 0) {
      printf("Error: reading CPU temperature.\n");
      return -1;
    }

    attempt++;
  }

  if (cpu_temp <= MAX_TEMPERATURE) {

    printf("[*] Temperature is below the limit set. Waiting for %d seconds to "
           "confirm...\n",
           WAIT_TIME_AFTER_TEMP_DROP);
    sleep(WAIT_TIME_AFTER_TEMP_DROP);
    cpu_temp = get_cpu_temp();

    if (cpu_temp <= MAX_TEMPERATURE) {
      printf("[*] Temperature is stable. Unthrottling...\n");
      return 1;
    } else {
      printf("[*] Temperature's still above limit set. Continuing "
             "throttling...\n");
      return throttle();
    }
  } else {
    printf("[!] Maximum attempts reached. Unthrottling and exiting...\n");
    return 0;
  }
}

void unthrottle() {
  int max_cpu = sysconf(_SC_NPROCESSORS_ONLN);

  for (int cpu = 0; cpu < max_cpu; cpu++) {
    int original_frequency = get_original_cpu_frequency(cpu);
    if (original_frequency < 0) {
      continue;
    }
    printf("CPU%d: Restoring frequency to original %d Hz\n", cpu,
           original_frequency);

    if (set_to_original_frequency(cpu, original_frequency) < 0) {
      continue;
    }
  }
}

int main() {
  float temp = get_cpu_temp();

  if (temp < 0) {
    printf("Error: reading CPU temperature.\n");
  } else if (temp > MAX_TEMPERATURE) {
    printf("[*] Starting Throttle...\n");
    int success = throttle();
    if (!success) {
      printf("[!] Throttling failed after max attempts. Unthrottling...\n");
    }
    unthrottle();
  } else {
    printf("[*] Temperature is fine: %.2f°C\n", temp);
  }

  return 0;
}
