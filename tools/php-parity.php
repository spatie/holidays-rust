<?php

// Run with: php tools/php-parity.php /path/to/spatie/holidays > tests/php-parity.json

use Carbon\CarbonImmutable;
use Spatie\Holidays\Contracts\HasRegions;
use Spatie\Holidays\CountryRegistry;
use Spatie\Holidays\Holidays;

$source = $argv[1] ?? throw new InvalidArgumentException('Pass the PHP package directory.');
require $source.'/vendor/autoload.php';

date_default_timezone_set('UTC');
CarbonImmutable::setTestNow('2024-01-01 00:00:00 UTC');
$years = range(1901, 2100);
$rows = [];

foreach (CountryRegistry::all() as $code => $class) {
    $country = $class::make();
    $regions = [null, ...(is_a($class, HasRegions::class, true) ? $class::regions() : [])];
    $locales = [null, 'unknown'];
    foreach (glob($source.'/lang/'.$country->countryCode().'/*/holidays.json') as $file) {
        $locales[] = basename(dirname($file));
    }

    foreach ($regions as $region) {
        foreach ($locales as $locale) {
            $hash = hash_init('sha256');
            $successfulYears = 0;
            foreach ($years as $year) {
                try {
                    $holidays = Holidays::for($code, $year, $locale, $region)->get();
                    $values = array_map(static fn ($holiday) => [
                        'date' => $holiday->date->toDateString(),
                        'name' => $holiday->name,
                        'region' => $holiday->region,
                        'type' => $holiday->type->value,
                    ], $holidays);
                    usort($values, static fn ($a, $b) => [$a['date'], $a['name'], $a['type'], $a['region'] ?? ''] <=> [$b['date'], $b['name'], $b['type'], $b['region'] ?? '']);
                    $encoded = json_encode($values, JSON_UNESCAPED_UNICODE | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_LINE_TERMINATORS | JSON_THROW_ON_ERROR);
                    $successfulYears++;
                } catch (Throwable $exception) {
                    $encoded = 'error';
                }
                hash_update($hash, $encoded."\n");
            }
            $rows[] = ['country' => $code, 'region' => $region, 'locale' => $locale, 'successful_years' => $successfulYears, 'sha256' => hash_final($hash)];
        }
    }
    fwrite(STDERR, $code."\n");
}

echo json_encode([
    'reference' => trim(shell_exec('git -C '.escapeshellarg($source).' rev-parse HEAD')),
    'icu' => INTL_ICU_VERSION,
    'timezone' => 'UTC',
    'clock' => '2024-01-01 00:00:00 UTC',
    'years' => $years,
    'rows' => $rows,
], JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_THROW_ON_ERROR)."\n";
