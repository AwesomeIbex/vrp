#[cfg(test)]
#[path = "../../../tests/unit/models/solution/tour_test.rs"]
mod tour_test;

use crate::models::problem::Job;
use crate::models::solution::Activity;
use hashbrown::HashSet;
use std::iter::once;
use std::ops::RangeBounds;
use std::slice::{Iter, IterMut};

pub type TourActivity = Box<Activity>;

/// Represents a tour, a smart container for jobs with their associated activities.
pub struct Tour {
    /// Stores activities in the order the performed.
    activities: Vec<TourActivity>,

    /// Stores jobs in the order of their activities added.
    jobs: HashSet<Job>,

    /// Keeps track whether tour is set as closed.
    is_closed: bool,
}

impl Default for Tour {
    fn default() -> Self {
        Tour { activities: Default::default(), jobs: Default::default(), is_closed: false }
    }
}

impl Tour {
    /// Sets tour start.
    pub fn set_start(&mut self, activity: TourActivity) -> &mut Tour {
        assert!(activity.job.is_none());
        assert!(self.activities.is_empty());
        self.activities.push(activity);

        self
    }

    /// Sets tour end.
    pub fn set_end(&mut self, activity: TourActivity) -> &mut Tour {
        assert!(activity.job.is_none());
        assert!(!self.activities.is_empty());
        self.activities.push(activity);
        self.is_closed = true;

        self
    }

    /// Inserts activity within its job to the end of tour.
    pub fn insert_last(&mut self, activity: TourActivity) -> &mut Tour {
        self.insert_at(activity, self.activity_count() + 1);
        self
    }

    /// Inserts activity within its job at specified index.
    pub fn insert_at(&mut self, activity: TourActivity, index: usize) -> &mut Tour {
        assert!(activity.job.is_some());
        assert!(!self.activities.is_empty());

        self.jobs.insert(activity.retrieve_job().unwrap());
        self.activities.insert(index, activity);

        self
    }

    /// Removes job within its activities from the tour.
    pub fn remove(&mut self, job: &Job) -> bool {
        self.activities.retain(|a| !a.has_same_job(job));
        self.jobs.remove(job)
    }

    /// Removes activity and its job from the tour.
    pub fn remove_activity_at(&mut self, idx: usize) -> Job {
        let job = self
            .activities
            .get(idx)
            .and_then(|a| a.retrieve_job())
            .expect("Attempt to remove activity without job from the tour!");
        self.remove(&job);

        job
    }

    /// Removes activities from the tour.
    pub fn remove_activities_at<R>(&mut self, range: R) -> Vec<Job>
    where
        R: RangeBounds<usize>,
    {
        let jobs: Vec<_> = self
            .activities
            .drain(range)
            .map(|a| a.retrieve_job().expect("Attempt to remove activity without job from the tour!"))
            .collect();

        jobs.iter().for_each(|job| {
            self.remove(job);
        });

        jobs
    }

    /// Returns all activities in tour.
    pub fn all_activities(&self) -> Iter<TourActivity> {
        self.activities.iter()
    }

    /// Returns activities slice in specific range (all inclusive).
    pub fn activities_slice(&self, start: usize, end: usize) -> &[TourActivity] {
        &self.activities[start..=end]
    }

    /// Returns all activities in tour as mutable.
    pub fn all_activities_mut(&mut self) -> IterMut<TourActivity> {
        self.activities.iter_mut()
    }

    /// Returns all activities in tour for specific job.
    pub fn job_activities<'a>(&'a self, job: &'a Job) -> impl Iterator<Item = &TourActivity> + 'a {
        self.activities.iter().filter(move |a| a.has_same_job(job))
    }

    /// Returns counted tour legs.
    pub fn legs<'a>(&'a self) -> Box<dyn Iterator<Item = (&'a [TourActivity], usize)> + 'a> {
        let last_index = self.activities.len() - 1;
        let window_size = if last_index == 0 { 1 } else { 2 };
        let legs = self.activities.windows(window_size).zip(0_usize..);

        let is_open_tour_with_jobs = !self.is_closed && last_index > 0;

        if is_open_tour_with_jobs {
            Box::new(legs.chain(once((&self.activities[last_index..], last_index))))
        } else {
            Box::new(legs)
        }
    }

    /// Returns all jobs.
    pub fn jobs<'a>(&'a self) -> impl Iterator<Item = Job> + 'a {
        self.jobs.iter().cloned()
    }

    /// Returns activity by its index in tour.
    pub fn get(&self, index: usize) -> Option<&TourActivity> {
        self.activities.get(index)
    }

    /// Returns mutable activity by its index in tour.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut TourActivity> {
        self.activities.get_mut(index)
    }

    /// Returns start activity in tour.
    pub fn start(&self) -> Option<&TourActivity> {
        self.activities.first()
    }

    /// Returns end activity in tour.
    pub fn end(&self) -> Option<&TourActivity> {
        self.activities.last()
    }

    /// Checks whether job is present in tour
    pub fn contains(&self, job: &Job) -> bool {
        self.jobs.contains(job)
    }

    /// Returns index of first job occurrence in the tour.
    pub fn index(&self, job: &Job) -> Option<usize> {
        self.activities.iter().position(move |a| a.has_same_job(&job))
    }

    /// Returns index of the activity in the tour.
    pub fn activity_index(&self, activity: &TourActivity) -> Option<usize> {
        self.activities
            .iter()
            .position(|a| (a.as_ref() as *const Activity as usize) == (activity.as_ref() as *const Activity as usize))
    }

    /// Checks whether tour has jobs.
    pub fn has_jobs(&self) -> bool {
        !self.jobs.is_empty()
    }

    /// Returns amount of job activities.
    pub fn activity_count(&self) -> usize {
        if self.activities.is_empty() {
            0
        } else {
            self.activities.len() - (if self.is_closed { 2 } else { 1 })
        }
    }

    /// Returns amount of all activities in tour.
    pub fn total(&self) -> usize {
        self.activities.len()
    }

    /// Returns amount of jobs.
    pub fn job_count(&self) -> usize {
        self.jobs.len()
    }

    /// Creates a copy of existing tour deeply copying all activities and jobs.
    pub fn deep_copy(&self) -> Tour {
        Tour {
            activities: self.activities.iter().map(|a| Box::new(a.deep_copy())).collect(),
            jobs: self.jobs.iter().cloned().collect(),
            is_closed: self.is_closed,
        }
    }
}
